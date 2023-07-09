use chess_bitboard::Pos::*;
use chess_movegen::{Board, ChessMove};

// const COUNT: u32 = 1_000_000;
const COUNT: u32 = 100_000_000;

fn main() {
    // [Event "Titled Tuesday Blitz January 03 Early 2023"]
    // [Site "chess.com"]
    // [Date "2023.01.03"]
    // [Round "?"]
    // [White "Hikaru Nakamura"]
    // [Black "Magnus Carlsen"]
    // [Result "1-0"]

    #[rustfmt::skip]
    let mvs = [
        (A2, A3), (G7, G6), (E2, E4), (C7, C5), (F1, C4),
        (F8, G7), (B1, C3), (B8, C6), (G1, E2), (E7, E6),
        (D2, D3), (G8, E7), (H2, H4), (H7, H5), (C1, G5),
        (D7, D6), (D1, D2), (D8, D7), (C4, A2), (B7, B6),
        (E1, C1), (C8, B7), (C1, B1), (E8, C8), (D2, F4),
        (F7, F6), (G5, F6), (H8, F8), (F6, G7), (F8, F4),
        (E2, F4), (D6, D5), (G7, F6), (D8, F8), (F6, G5),
        (C6, D4), (F2, F3), (B6, B5), (G5, E7), (D7, E7),
        (F4, G6), (E7, G7), (G6, F8), (G7, F8), (H1, E1),
        (F8, G8), (E4, D5), (G8, G2), (D5, E6), (G2, C2),
        (B1, A1), (B7, C6), (E6, E7), (C6, E8), (C3, E4),
        (C2, H2), (E1, H1), (H2, E5), (D1, C1), (C5, C4),
        (D3, C4), (B5, B4), (A3, B4), (E5, E7), (C4, C5),
        (E8, A4), (H1, G1), (D4, C2), (A1, B1), (C2, B4),
        (E4, D6), (C8, D7), (C1, C4), (E7, E3), (C5, C6),
        (A4, C6), (G1, D1), (E3, E2), (C4, C1), (B4, D3),
        (D6, C4), (D7, C7), (D1, D2), (E2, F3), (C1, D1),
        (C6, E4), (B1, A1), (F3, F4), (A2, B1), (D3, C5),
        (B1, A2), (E4, C6), (D1, C1), (C6, B5), (C1, C3),
    ];

    let mvs = mvs.map(|(source, dest)| ChessMove {
        source,
        dest,
        promotion: None,
    });

    let ext = run_chess_ext(mvs);
    let mine = run_chess(mvs);

    eprintln!("MINE {:.1}% faster", 100.0 - 100.0 * mine / ext)
}

fn run_chess<const N: usize>(mvs: [ChessMove; N]) -> f64 {
    {
        let mut board = Board::standard();
        for mv in mvs {
            unsafe { board.move_unchecked_mut(mv) }
        }
        eprintln!("{board}")
    }

    let start = std::time::Instant::now();
    for _ in 0..COUNT {
        let mut board = Board::standard();
        for mv in mvs {
            std::hint::black_box(&mut board);
            unsafe { board.move_unchecked_mut(mv) }
        }
    }
    let time = start.elapsed().as_secs_f64() / f64::from(COUNT) / N as f64 * 1e9;
    eprintln!("MINE = {time}");
    time
}

fn run_chess_ext<const N: usize>(mvs: [ChessMove; N]) -> f64 {
    let board: chess::Board = chess::Board::default();
    let mvs = mvs.map(|mv| {
        chess::ChessMove::new(
            chess::Square::make_square(
                chess::Rank::from_index(mv.source.rank() as usize),
                chess::File::from_index(mv.source.file() as usize),
            ),
            chess::Square::make_square(
                chess::Rank::from_index(mv.dest.rank() as usize),
                chess::File::from_index(mv.dest.file() as usize),
            ),
            mv.promotion.map(|piece| match piece {
                chess_bitboard::PromotionPiece::Knight => chess::Piece::Knight,
                chess_bitboard::PromotionPiece::Bishop => chess::Piece::Bishop,
                chess_bitboard::PromotionPiece::Rook => chess::Piece::Rook,
                chess_bitboard::PromotionPiece::Queen => chess::Piece::Queen,
            }),
        )
    });

    {
        let mut board = board;
        for mv in mvs {
            let old = board;
            assert!(old.legal(mv));
            old.make_move(mv, &mut board);
        }
        eprintln!("{board}")
    }

    let start = std::time::Instant::now();
    for _ in 0..COUNT {
        let mut board = board;
        for mv in mvs {
            let old = board;
            old.make_move(mv, &mut board);
        }
    }
    let time = start.elapsed().as_secs_f64() / f64::from(COUNT) / N as f64 * 1e9;
    eprintln!(" EXT = {time}");
    time
}
