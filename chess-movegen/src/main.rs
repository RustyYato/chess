use chess_bitboard::Pos;
use chess_movegen::{Board, ChessMove};

fn main() {
    let mut board = Board::standard();

    eprintln!("{board:?}");

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

    for mv in mvs {
        unsafe { board.move_unchecked_mut(mv) }
        eprintln!("{mv:?}");
        eprintln!("{board:?}");
    }
}
