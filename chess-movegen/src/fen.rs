use chess_bitboard::{Color, File, Piece, Pos, Rank, Side};

use crate::{castle_rights::CastleRights, raw};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseFenError {
    InvalidPiece(u8, Pos),
    MissingPiece(Pos),
    MissingWhitespace(MissingWhitespace),
    InvalidTurn(u8),
    MissingTurn,
    FileOutOfBounds(Rank),
    InvalidEnpassant { file: u8, rank: u8 },
    MissingEnpassant,
    MissingCastleRights,
    MissingHalfClock,
    MissingFullClock,
    TrailingBytes,
    BoardValidation(crate::BoardValidationError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MissingWhitespace {
    Pieces,
    Turn,
    CastleRights,
    Enpassant,
    HalfMoveClock,
}

pub fn parse_fen(mut s: &[u8]) -> Result<crate::Board, ParseFenError> {
    let mut file = 0;
    let mut ranks = Rank::all().rev();
    let mut rank = ranks.next().unwrap();

    let mut board = raw::RawBoard::empty();
    let mut zobrist = 0;

    loop {
        let (out, rest) = parse_piece(s);
        s = rest;

        let pos = Pos::new(File::from_u8(file).unwrap(), rank);

        let dist = match out {
            Some(Ok((color, piece))) => {
                board.set_unchecked(color, piece, pos);
                zobrist ^= chess_lookup::zobrist(pos, piece, color);
                1
            }
            Some(Err(dist)) => dist,
            None => match s {
                [b'/', rest @ ..] => {
                    s = rest;
                    0
                }
                [b' ', rest @ ..] => {
                    s = rest;
                    continue;
                }
                [x, ..] => return Err(ParseFenError::InvalidPiece(*x, pos)),
                [] => return Err(ParseFenError::MissingPiece(pos)),
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
            9.. => return Err(ParseFenError::FileOutOfBounds(rank)),
        }
    }

    s = parse_whitespace(s, MissingWhitespace::Pieces)?;
    let (turn, mut s) = match s {
        [b'b', s @ ..] => (Color::Black, s),
        [b'w', s @ ..] => (Color::White, s),
        &[turn, ..] => return Err(ParseFenError::InvalidTurn(turn)),
        [] => return Err(ParseFenError::MissingTurn),
    };

    s = parse_whitespace(s, MissingWhitespace::Turn)?;

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
        s = parse_dash(s).ok_or(ParseFenError::MissingCastleRights)?;
    }

    s = parse_whitespace(s, MissingWhitespace::CastleRights)?;

    let (enpassant_target, mut s) = match s {
        &[file @ (b'a'..=b'h'), rank @ (b'3' | b'6'), ref s @ ..] => {
            let expected_rank = match turn {
                Color::White => b'6',
                Color::Black => b'3',
            };

            if rank != expected_rank {
                return Err(ParseFenError::InvalidEnpassant { file, rank });
            }

            (Some(File::from_u8(file - b'a').unwrap()), s)
        }
        [b'-', s @ ..] => (None, s),
        &[file, rank, ..] => return Err(ParseFenError::InvalidEnpassant { file, rank }),
        [_] | [] => return Err(ParseFenError::MissingEnpassant),
    };

    s = parse_whitespace(s, MissingWhitespace::Enpassant)?;

    let half_move_clock = parse_number(&mut s).ok_or(ParseFenError::MissingHalfClock)?;
    s = parse_whitespace(s, MissingWhitespace::HalfMoveClock)?;
    let full_move_clock = parse_number(&mut s).ok_or(ParseFenError::MissingFullClock)?;

    let mut board = crate::Board {
        zobrist,
        raw: board,
        turn,
        pinned: chess_bitboard::BitBoard::empty(),
        checkers: chess_bitboard::BitBoard::empty(),
        castle_rights,
        enpassant_target: enpassant_target.into(),
        half_move_clock,
        full_move_clock,
    };

    if let Err(err) = board.validate() {
        return Err(ParseFenError::BoardValidation(err));
    }

    if s.is_empty() {
        board.update_pin_info();
        Ok(board)
    } else {
        Err(ParseFenError::TrailingBytes)
    }
}

#[inline(always)]
fn parse_number(s: &mut &[u8]) -> Option<u16> {
    let mut num = 0;
    for i in 0..4 {
        match *s {
            [d @ (b'0'..=b'9'), r @ ..] => {
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

#[inline]
fn parse_whitespace(mut s: &[u8], err: MissingWhitespace) -> Result<&[u8], ParseFenError> {
    let mut has_whitespace = false;
    while let [b' ', r @ ..] = s {
        s = r;
        has_whitespace = true;
    }
    if has_whitespace {
        Ok(s)
    } else {
        Err(ParseFenError::MissingWhitespace(err))
    }
}

#[inline]
fn parse_dash(s: &[u8]) -> Option<&[u8]> {
    match s {
        [b'-', s @ ..] => Some(s),
        _ => None,
    }
}

#[inline]
fn parse_castle_rights(s: &[u8], b: u8) -> (bool, &[u8]) {
    match s {
        [x, s @ ..] if *x == b => (true, s),
        _ => (false, s),
    }
}

#[inline]
#[allow(clippy::type_complexity)]
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
