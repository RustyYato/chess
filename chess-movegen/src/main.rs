use chess_bitboard::Pos;
use chess_movegen::{Board, ChessMove};

const COUNT: u32 = 100_000_000;

fn main() {
    // eprintln!("{board:?}");

    let mvs = [
        ChessMove {
            source: Pos::E2,
            dest: Pos::E4,
            promotion: None,
        },
        ChessMove {
            source: Pos::E7,
            dest: Pos::E5,
            promotion: None,
        },
        ChessMove {
            source: Pos::D1,
            dest: Pos::H5,
            promotion: None,
        },
    ];

    run_chess(mvs);
    run_chess_ext(mvs);
}

fn run_chess<const N: usize>(mvs: [ChessMove; N]) {
    {
        let mut board = Board::standard();
        for mv in mvs {
            unsafe { board.move_unchecked_mut(mv) }
            eprintln!("{board}")
        }
    }

    let start = std::time::Instant::now();
    for _ in 0..COUNT {
        let mut board = Board::standard();
        for mv in mvs {
            std::hint::black_box(&mut board);
            unsafe { board.move_unchecked_mut(mv) }
        }
    }
    eprintln!("MINE = {:?}", start.elapsed() / COUNT)
}

fn run_chess_ext<const N: usize>(mvs: [ChessMove; N]) {
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
            old.make_move(mv, &mut board);
            eprintln!("{board}")
        }
    }

    let start = std::time::Instant::now();
    for _ in 0..COUNT {
        let mut board = board;
        for mv in mvs {
            let old = board;
            old.make_move(mv, &mut board);
        }
    }
    eprintln!(" EXT = {:?}", start.elapsed() / COUNT)
}
