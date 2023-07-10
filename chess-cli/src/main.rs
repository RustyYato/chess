use std::time::Duration;

use chess_engine::{DurationTimeout, Engine};
use chess_movegen::Board;

fn main() {
    let mut engine = Engine::default();

    let board = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 0";
    // let board = "r3k2r/p1ppqpb1/Bn2pnp1/3PN3/4P3/2p2Q1p/PPPB1PPP/R3K2R w KQkq - 0 1";
    // let board = "2k5/8/2K5/8/8/8/6R1/8 w - - 0 1";
    // let board = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 0";

    let mut board = board.parse::<Board>().unwrap();

    for _ in 0..4 {
        eprintln!("{board}");
        eprintln!("{board:?}");

        // let start = std::time::Instant::now();
        let (mv, score) = engine.search(&board, &DurationTimeout::new(Duration::from_millis(1000)));
        // dbg!(start.elapsed());
        let mv = mv.unwrap();
        assert!(board.move_mut(mv));
        eprintln!("{score:?} {mv:?} moves: {}", engine.moves_evaluated);
    }

    eprintln!("{board:?}");
}
