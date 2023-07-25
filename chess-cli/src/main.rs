use std::time::Duration;

use chess_engine::{DurationTimeout, Engine, ThreeFold};
use chess_movegen::Board;

use rand::{prelude::IteratorRandom, Rng};

mod bot_fight;
mod logs;
mod make_bot;

#[derive(clap::Parser)]
struct Args {
    #[clap(global = true, long, short, action = clap::ArgAction::Count, conflicts_with = "quiet")]
    verbose: u8,
    #[clap(global = true, long, short, action = clap::ArgAction::Count, conflicts_with = "verbose")]
    quiet: u8,
    #[clap(subcommand)]
    kind: ArgKind,
}

#[derive(Clone, clap::Parser)]
enum ArgKind {
    OnBoard { board: Option<Board> },
    BotFight(bot_fight::Args),
    MakeBot(make_bot::Args),
}

fn main() {
    let args: Args = clap::Parser::parse();

    logs::init(args.verbose as i8 - args.quiet as i8);

    match args.kind {
        ArgKind::BotFight(args) => bot_fight::main(args),
        ArgKind::MakeBot(args) => make_bot::main(args),
        ArgKind::OnBoard { board } => {
            let mut engine = Engine::default();
            let mut three_fold = ThreeFold::new();
            let mut book_moves = if board.is_none() {
                chess_lookup::INITIAL_BOOOK_MOVES
            } else {
                chess_lookup::EMPTY_BOOK_MOVES
            };

            let mut board = board.unwrap_or_else(Board::standard);

            loop {
                let x = book_moves.into_iter().count();
                if x == 0 {
                    break;
                }

                let x = rand::thread_rng()
                    .sample(rand::distributions::WeightedIndex::new((1..=x).rev()).unwrap());
                let mv = book_moves.into_iter().nth(x).unwrap();

                eprintln!("{board:?}");
                assert!(board.move_mut(chess_movegen::ChessMove {
                    source: mv.source,
                    dest: mv.dest,
                    piece: None,
                }));
                book_moves = mv.children;
            }

            loop {
                eprintln!("{board}");
                eprintln!("{board:?}");

                // let start = std::time::Instant::now();
                let (mv, score) = engine.search(
                    &board,
                    &three_fold,
                    DurationTimeout::new(Duration::from_millis(5000)),
                );

                let Some(mv) = mv else {
                    println!("DRAW (MATERIAL)");
                    break;
                };
                // dbg!(start.elapsed());
                assert!(board.move_mut(mv));
                eprintln!(
                    "{score:?} {mv} moves: {}, max_depth: {}",
                    engine.moves_evaluated, engine.max_depth
                );

                if three_fold.add(board) {
                    println!("DRAW (THREE FOLD)");
                    break;
                }

                if board.legals().is_empty() {
                    if board.in_check() {
                        println!("WIN");
                    } else {
                        println!("DRAW (NO LEGAL MOVES)");
                    }
                    break;
                }
            }

            eprintln!("{board:?}");
        }
    }
}
