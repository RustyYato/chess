use std::{collections::HashMap, time::Duration};

use chess_engine::{DurationTimeout, Engine};
use chess_movegen::Board;

fn main() {
    let mut engine = Engine::default();

    let board = "6k1/8/8/4K3/8/8/8/3q4 w - - 0 1";
    // let board = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 0";
    // let board = "r3k2r/p1ppqpb1/Bn2pnp1/3PN3/4P3/2p2Q1p/PPPB1PPP/R3K2R w KQkq - 0 1";
    // let board = "2k5/8/2K5/8/8/8/6R1/8 w - - 0 1";
    // let board = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 0";

    let mut board = board.parse::<Board>().unwrap();

    let mut prev_boards = HashMap::new();

    loop {
        eprintln!("{board}");
        eprintln!("{board:?}");

        // let start = std::time::Instant::now();
        let (mv, score) = engine.search(&board, &DurationTimeout::new(Duration::from_millis(100)));
        // dbg!(start.elapsed());
        let mv = mv.unwrap();
        assert!(board.move_mut(mv));
        eprintln!("{score:?} {mv:?} moves: {}", engine.moves_evaluated);

        let count = prev_boards.entry(board).or_insert(0);

        *count += 1;

        if *count == 3 {
            println!("DRAW (THREE FOLD)");
            break;
        }

        if board.legals().len() == 0 {
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
