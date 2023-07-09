use chess_bitboard::{Color, File, Piece, Pos, Rank, Side};

use crate::{castle_rights::CastleRights, raw};

fn parse_fen(mut s: &[u8]) -> crate::Board {
    let mut file = 0;
    let mut ranks = Rank::all().rev();
    let mut rank = ranks.next().unwrap();

    let mut board = raw::RawBoard::empty();

    loop {
        let (out, rest) = parse_piece(s);
        s = rest;
        let dist = match out {
            Some(Ok((color, piece))) => {
                let file = File::from_u8(file).unwrap();
                let pos = Pos::new(file, rank);

                board.set(color, piece, pos).unwrap();
                1
            }
            Some(Err(dist)) => dist,
            None => match s {
                [b'/', rest @ ..] => {
                    s = rest;
                    0
                }
                _ => todo!("{}", s[0] as char),
            },
        };

        file += dist;

        match file {
            ..=7 => {
                //
            }
            8 => {
                let Some(r) = ranks.next() else {
                    break
                };

                file = 0;
                rank = r;
            }
            9.. => panic!(),
        }
    }

    s = parse_whitespace(s).unwrap();
    let (turn, mut s) = match s {
        [b'b', s @ ..] => (Color::Black, s),
        [b'w', s @ ..] => (Color::White, s),
        _ => unimplemented!(),
    };

    s = parse_whitespace(s).unwrap();

    let mut castle_rights = CastleRights::empty();

    let (white_king_cr, s) = parse_castle_rights(s, b'K');
    let (white_queen_cr, s) = parse_castle_rights(s, b'Q');
    let (black_king_cr, s) = parse_castle_rights(s, b'k');
    let (black_queen_cr, mut s) = parse_castle_rights(s, b'q');

    if white_king_cr {
        castle_rights.add(Side::King, Color::White);
    }

    if white_queen_cr {
        castle_rights.add(Side::Queen, Color::White);
    }

    if black_king_cr {
        castle_rights.add(Side::King, Color::Black);
    }

    if black_queen_cr {
        castle_rights.add(Side::Queen, Color::Black);
    }

    if !(white_king_cr || white_queen_cr || black_king_cr || black_queen_cr) {
        s = parse_dash(s).unwrap();
    }

    s = parse_whitespace(s).unwrap();

    let (enpassant_target, mut s) = match s {
        [file @ (b'a'..=b'h'), rank @ (b'2' | b'6'), s @ ..] => {
            let expected_rank = match turn {
                Color::White => 6,
                Color::Black => 2,
            };

            assert_eq!(*rank, expected_rank);

            (Some(File::from_u8(*file - b'a').unwrap()), s)
        }
        [b'-', s @ ..] => (None, s),
        _ => todo!(),
    };

    s = parse_whitespace(s).unwrap();

    let half_move_clock = parse_number(&mut s).unwrap();
    s = parse_whitespace(s).unwrap();
    let full_move_clock = parse_number(&mut s).unwrap();

    let mut board = crate::Board {
        raw: board,
        turn,
        pinned: chess_bitboard::BitBoard::empty(),
        checkers: chess_bitboard::BitBoard::empty(),
        castle_rights,
        enpassant_target,
        half_move_clock,
        full_move_clock,
    };

    if s.is_empty() {
        board.update_pin_info();
        board
    } else {
        panic!("Invalid FEN string, too much input")
    }
}

fn parse_number(s: &mut &[u8]) -> Option<u16> {
    let mut num = 0;
    for i in 0..4 {
        match *s {
            [d @ (b'0'..=b'9'), r @ ..] => {
                if *d == b'0' && i != 0 {
                    panic!("no leading zeros")
                }
                num *= 10;
                num += u16::from(d - b'0');
                *s = r;
            }
            _ => {
                if i == 0 {
                    return None;
                }
            }
        }
    }

    Some(num)
}

fn parse_whitespace(mut s: &[u8]) -> Option<&[u8]> {
    let mut has_whitespace = false;
    while let [b' ', r @ ..] = s {
        s = r;
        has_whitespace = true;
    }
    if has_whitespace {
        Some(s)
    } else {
        None
    }
}

fn parse_dash(mut s: &[u8]) -> Option<&[u8]> {
    match s {
        [b'-', s @ ..] => Some(s),
        _ => None,
    }
}

fn parse_castle_rights(mut s: &[u8], b: u8) -> (bool, &[u8]) {
    match s {
        [x, s @ ..] if *x == b => (true, s),
        _ => (false, s),
    }
}

fn parse_piece(s: &[u8]) -> (Option<Result<(Color, Piece), u8>>, &[u8]) {
    match s {
        [b'p', rest @ ..] => (Some(Ok((Color::Black, Piece::Pawn))), rest),
        [b'n', rest @ ..] => (Some(Ok((Color::Black, Piece::Knight))), rest),
        [b'b', rest @ ..] => (Some(Ok((Color::Black, Piece::Bishop))), rest),
        [b'r', rest @ ..] => (Some(Ok((Color::Black, Piece::Rook))), rest),
        [b'q', rest @ ..] => (Some(Ok((Color::Black, Piece::Queen))), rest),
        [b'k', rest @ ..] => (Some(Ok((Color::Black, Piece::King))), rest),

        [b'P', rest @ ..] => (Some(Ok((Color::White, Piece::Pawn))), rest),
        [b'N', rest @ ..] => (Some(Ok((Color::White, Piece::Knight))), rest),
        [b'B', rest @ ..] => (Some(Ok((Color::White, Piece::Bishop))), rest),
        [b'R', rest @ ..] => (Some(Ok((Color::White, Piece::Rook))), rest),
        [b'Q', rest @ ..] => (Some(Ok((Color::White, Piece::Queen))), rest),
        [b'K', rest @ ..] => (Some(Ok((Color::White, Piece::King))), rest),

        [dist @ (b'1'..=b'8'), rest @ ..] => (Some(Err(*dist - b'0')), rest),
        _ => (None, s),
    }
}

#[test]
fn test() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 0";

    let board = parse_fen(fen.as_bytes());

    eprintln!("{board:?}");
    panic!()
}
