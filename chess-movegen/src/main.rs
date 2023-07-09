use chess_movegen::Board;

fn main() {
    let board = Board::standard();

    eprintln!("{:x}", board.raw());

    let start = std::time::Instant::now();
    for _ in 0..10_000_000 {
        Board::standard();
        // "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 0"
        //     .parse::<Board>()
        //     .unwrap();
    }
    dbg!(start.elapsed() / 10_000_000);
}
