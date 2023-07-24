use chess_movegen::Board;
use colorz::Colorize;
use rand::seq::SliceRandom;
use std::{collections::HashMap, path::PathBuf, time::Duration};

use rayon::prelude::*;

#[derive(Clone, clap::Parser)]
pub struct Args {
    #[clap(required = true, num_args = 2..)]
    bots: Vec<PathBuf>,
    #[clap(short, long)]
    games: Vec<u32>,
    #[clap(short, long, value_parser(parse_duration::parse))]
    time_controls: Vec<Duration>,
    #[clap(long, env = "RAYON_NUM_THREADS", default_value_t = 4)]
    thread_count: usize,
}

#[allow(dead_code)]
#[derive(Debug)]
enum GameResult {
    CheckMate { winner: usize, loser: usize },
    StaleMate { x: usize, y: usize },
    DidntMove { bot_id: usize, opp_id: usize },
}

pub fn main(args: Args) {
    if args.games.len() != args.time_controls.len() {
        eprintln!("Invalid number of games or time controls");
        std::process::exit(1);
    }

    let mut bot_apis = Vec::<chess_api::ChessApiRef>::new();

    let games = args
        .games
        .into_iter()
        .zip(args.time_controls)
        .collect::<Vec<_>>();

    for bot_path in &args.bots {
        let bot = match chess_api::ChessApiRef::load_from_file(bot_path) {
            Ok(bot) => bot,
            Err(err) => {
                eprintln!("Could load bot at {}, {err}", bot_path.display());
                std::process::exit(1);
            }
        };

        bot_apis.push(bot);
    }

    let indicies = 0..bot_apis.len();

    rayon::ThreadPoolBuilder::new()
        .num_threads(args.thread_count)
        .build_global()
        .unwrap();

    let mut game_specs = indicies
        .clone()
        .flat_map(|x| indicies.clone().map(move |y| (x, y)))
        .filter(|(x, y)| x != y)
        .flat_map(|(x, y)| {
            games
                .iter()
                .map(move |&(games, time_control)| (x, y, games, time_control))
        })
        .flat_map(|(x, y, games, time_control)| (0..games).map(move |_| (x, y, time_control)))
        .collect::<Vec<_>>();

    game_specs.shuffle(&mut rand::thread_rng());

    let results = game_specs.into_par_iter()
        .map(|(x, y, time_control)| {
            tracing::info!(
                "started game between {} ({x}) and {} ({y}) at {time_control:?} per move",
                args.bots[x].display(),
                args.bots[y].display()
            );

            let start = std::time::Instant::now();
            let mut a = bot_apis[x].new_engine();
            let mut b = bot_apis[y].new_engine();

            a.set_board(Board::standard());
            b.set_board(Board::standard());
            let mut moves = Vec::new();

            let result = loop {
                let timeout = chess_engine::DurationTimeout::new(time_control);
                let (bot, bot_id, opp_id) = match a.board().turn() {
                    chess_bitboard::Color::White => (&mut a,x,y),
                    chess_bitboard::Color::Black => (&mut b,y,x),
                };
                let (mv, _score) = bot.evaluate(&timeout);

                if let Some(mv) = mv {
                    moves.push(mv);
                    let res = a.make_move(mv);
                    b.make_move(mv);

                    if res.is_three_fold_draw {
                        break GameResult::StaleMate { x: bot_id, y: opp_id };
                    }
                } else {
                    tracing::error!(
                        bot=?args.bots[bot_id],
                        ?time_control,
                        "{}", "didn't move".red()
                    );
                    break GameResult::DidntMove { bot_id, opp_id };
                }

                match a.board().state() {
                    chess_movegen::GameState::CheckMate => break GameResult::CheckMate { winner: bot_id, loser:opp_id },
                     chess_movegen::GameState::StaleMate => break GameResult::StaleMate { x: bot_id, y: opp_id },
                    chess_movegen::GameState::Check | chess_movegen::GameState::Running => (),
                }
            };
            let end = start.elapsed();


            tracing::info!(
                x,
                y,
                x.path=?args.bots[x].display(),
                y.path=?args.bots[y].display(),
                duratin=?end,
                moves.len=moves.len(),
                "completed game between {} ({x}) and {} ({y}) at {time_control:?} per move after {} moves as a {result:?} in {end:?}",
                args.bots[x].display(),
                args.bots[y].display(),
                moves.len()
            );

            let game_id = if x < y {
                ((x, y), time_control)
            } else {
                ((y, x), time_control)
            };

            (game_id, result)
        })
        .fold(HashMap::new, |mut acc, ((game_id, time_control), x)| {
            let games=    acc.entry(game_id).or_insert_with(Vec::<(Duration, u32, u32, u32)>::new);

            let (x_win, y_win, ties)=if let Some((_, x_win, y_win, ties)) = games.iter_mut().find(|(tc, ..)| *tc ==time_control) {
                (x_win, y_win, ties)
            } else {
                games.push((time_control,0,0,0));
                let (_, x_win, y_win, ties) = games.last_mut().unwrap();
                (x_win, y_win, ties)
            };

            match x {
                GameResult::CheckMate { winner, loser: _ } => {
                    if game_id.0 == winner {
                        *x_win += 1;
                    } else {
                        *y_win += 1;
                    }
                },
                GameResult::StaleMate { .. } => *ties += 1,
                GameResult::DidntMove { .. } => (),
            }

            acc
        }).reduce(HashMap::new, |mut left, right| {
            for (id, results) in right {
                let games = left.entry(id)
                    .or_insert_with(Vec::new);

                for (time_control, x_wins, y_wins, ties) in results {
                    let (x_win_, y_win_, ties_) = if let Some((_, x_win, y_win, ties)) = games.iter_mut().find(|(tc, ..)| *tc == time_control) {
                        (x_win, y_win, ties)
                    } else {
                        games.push((time_control, 0, 0, 0));
                        let (_, x_win, y_win, ties) = games.last_mut().unwrap();
                        (x_win, y_win, ties)
                    };

                    *x_win_ += x_wins;
                    *y_win_ += y_wins;
                    *ties_ += ties;
                }
            }

            left
        });

    eprintln!("Results:");

    for ((x, y), results) in results {
        eprintln!("\t{} vs {}", args.bots[x].display(), args.bots[y].display());

        for (time_control, x_wins, y_wins, ties) in results {
            eprintln!("\t\t{time_control:?}\t{x_wins}\t{y_wins}\t{ties}");
        }
    }
}
