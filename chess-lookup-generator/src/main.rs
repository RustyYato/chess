#![allow(unused)]

use std::{error::Error, fs::File, io::BufWriter, io::Write, path::Path};

use chess_bitboard::{Color, File as ChessFile, Piece, Pos};
use chess_lookup_generator::MagicTable;

fn main() -> Result<(), Box<dyn Error>> {
    let target_dir = std::env::args_os().nth(1);

    let target_dir = match target_dir {
        None => {
            eprintln!("usage: chess-lookup-generator <target-dir>");
            return Err("")?;
        }
        Some(target_dir) => target_dir,
    };

    let target_dir = Path::new(&target_dir);

    #[cfg(not(miri))]
    assert!(target_dir.exists());

    // write_rook_rays(target_dir)?;
    // write_bishop_rays(target_dir)?;
    // write_knight_moves(target_dir)?;
    // write_king_moves(target_dir)?;
    // write_pawn_moves(target_dir)?;
    // write_between(target_dir)?;
    // write_line(target_dir)?;
    // write_bishop_moves(target_dir)?;
    // write_rook_moves(target_dir)?;
    // write_zobrist(target_dir)?;
    // #[cfg(feature = "book")]
    // write_openning_book(target_dir)?;
    #[cfg(feature = "book")]
    write_lichess_openning_book(target_dir)?;

    Ok(())
}

#[cfg(feature = "book")]
fn write_openning_book(target_dir: &Path) -> Result<(), Box<dyn Error>> {
    let (book_data, names) = chess_lookup_generator::eco_book::read_eco()?;

    let mut book = BufWriter::new(File::create(target_dir.join("book.rs"))?);

    write!(
        book,
        "
pub(super) const BOOK_SIZE: usize = {0};
pub(super) static BOOK: [u16; {0}] = {1:?};
pub(super) static BOOK_NAMES: [&str; {2}] = {3:?};",
        book_data.len(),
        book_data,
        names.len(),
        names,
    )?;

    Ok(())
}

#[cfg(feature = "book")]
fn write_lichess_openning_book(target_dir: &Path) -> Result<(), Box<dyn Error>> {
    let book_data = chess_lookup_generator::book::read_lichess_games()?;

    let mut book = BufWriter::new(File::create(target_dir.join("lichess_book.rs"))?);

    write!(
        book,
        "
pub(super) const BOOK_SIZE: usize = {0};
pub(super) static BOOK: [u16; {0}] = {1:?};",
        book_data.len(),
        book_data,
    )?;

    Ok(())
}

fn write_zobrist(target_dir: &Path) -> Result<(), Box<dyn Error>> {
    let mut zobrist = BufWriter::new(File::create(target_dir.join("zobrist.rs"))?);

    writeln!(
        zobrist,
        "pub(super) static PIECE_ZOBRIST: [[[u64; 6]; 64]; 2] = ["
    )?;
    for _ in Color::all() {
        write!(zobrist, "[")?;
        for _ in Pos::all() {
            write!(zobrist, "[")?;
            for _ in Piece::all() {
                write!(zobrist, "0x{:x},", rand::random::<u64>())?
            }
            write!(zobrist, "],")?
        }
        write!(zobrist, "],")?
    }
    writeln!(zobrist, "];")?;

    writeln!(zobrist, "pub(super) static CASTLE_ZOBRIST: [u64; 16] = [")?;

    for _ in 0..16 {
        write!(zobrist, "0x{:x},", rand::random::<u64>())?
    }
    writeln!(zobrist, "];")?;

    writeln!(
        zobrist,
        "pub(super) static EN_PASSANT_ZOBRIST: [u64; 8] = ["
    )?;

    for _ in ChessFile::all() {
        write!(zobrist, "0x{:x},", rand::random::<u64>())?
    }
    writeln!(zobrist, "];")?;

    writeln!(zobrist, "pub(super) static TURN_ZOBRIST: [u64; 2] = [")?;

    for _ in Color::all() {
        write!(zobrist, "0x{:x},", rand::random::<u64>())?
    }
    writeln!(zobrist, "];")?;

    Ok(())
}

fn write_rook_rays(target_dir: &Path) -> Result<(), Box<dyn Error>> {
    let mut all_rays = BufWriter::new(File::create(target_dir.join("rook_rays.rs"))?);

    writeln!(all_rays, "pub(super) static RAYS: [u64; 64] = [")?;
    for pos in Pos::all() {
        let rays = chess_lookup_generator::rook_rays(pos);
        writeln!(all_rays, "    0x{:x},", rays.to_u64())?;
    }
    writeln!(all_rays, "];")?;
    Ok(())
}

fn write_bishop_rays(target_dir: &Path) -> Result<(), Box<dyn Error>> {
    let mut all_rays = BufWriter::new(File::create(target_dir.join("bishop_rays.rs"))?);

    writeln!(all_rays, "pub(super) static RAYS: [u64; 64] = [")?;
    for pos in Pos::all() {
        let rays = chess_lookup_generator::bishop_rays(pos);
        writeln!(all_rays, "    0x{:x},", rays.to_u64())?;
    }
    writeln!(all_rays, "];")?;
    Ok(())
}

fn write_knight_moves(target_dir: &Path) -> Result<(), Box<dyn Error>> {
    let mut all_rays = BufWriter::new(File::create(target_dir.join("knight_moves.rs"))?);

    writeln!(all_rays, "pub(super) static MOVES: [u64; 64] = [")?;
    for pos in Pos::all() {
        let rays = chess_lookup_generator::knight_moves(pos);
        writeln!(all_rays, "    0x{:x},", rays.to_u64())?;
    }
    writeln!(all_rays, "];")?;
    Ok(())
}

fn write_king_moves(target_dir: &Path) -> Result<(), Box<dyn Error>> {
    let mut all_rays = BufWriter::new(File::create(target_dir.join("king_moves.rs"))?);

    writeln!(all_rays, "pub(super) static MOVES: [u64; 64] = [")?;
    for pos in Pos::all() {
        let rays = chess_lookup_generator::king_moves(pos);
        writeln!(all_rays, "    0x{:x},", rays.to_u64())?;
    }
    writeln!(all_rays, "];")?;
    Ok(())
}

fn write_pawn_moves(target_dir: &Path) -> Result<(), Box<dyn Error>> {
    let mut all_rays = BufWriter::new(File::create(target_dir.join("pawn.rs"))?);

    writeln!(
        all_rays,
        "pub(super) static PAWN_ATTACKS: [[u64; 2]; 64] = ["
    )?;
    for pos in Pos::all() {
        let [white, black] = chess_lookup_generator::pawn_attacks(pos);
        writeln!(
            all_rays,
            "    [0x{:x}, 0x{:x}],",
            white.to_u64(),
            black.to_u64()
        )?;
    }
    writeln!(all_rays, "];")?;

    writeln!(
        all_rays,
        "pub(super) static PAWN_QUIETS: [[u64; 2]; 64] = ["
    )?;
    for pos in Pos::all() {
        let [white, black] = chess_lookup_generator::pawn_quiets(pos);
        writeln!(
            all_rays,
            "    [0x{:x}, 0x{:x}],",
            white.to_u64(),
            black.to_u64()
        )?;
    }
    writeln!(all_rays, "];")?;
    Ok(())
}

fn write_rook_moves(target_dir: &Path) -> Result<(), Box<dyn Error>> {
    let table = chess_lookup_generator::rook_moves();
    let moves_table = BufWriter::new(File::create(target_dir.join("rook_moves.rs"))?);
    write_magic_table(table, moves_table)
}

fn write_bishop_moves(target_dir: &Path) -> Result<(), Box<dyn Error>> {
    let table = chess_lookup_generator::bishop_moves();
    let moves_table = BufWriter::new(File::create(target_dir.join("bishop_moves.rs"))?);
    write_magic_table(table, moves_table)
}

fn write_magic_table<W: std::io::Write>(table: MagicTable, mut f: W) -> Result<(), Box<dyn Error>> {
    writeln!(f, "use super::Magic;")?;
    writeln!(f, "pub(super) static MOVES_MAGIC: [Magic; 64] = [")?;
    for entry in table.entries {
        writeln!(
            f,
            "    Magic {{ factor: 0x{:x}, mask: 0x{:x}, offset: {}, shift: {} }},",
            entry.factor,
            entry.mask.to_u64(),
            entry.offset,
            entry.shift
        )?;
    }
    writeln!(f, "];")?;
    writeln!(
        f,
        "pub(super) static SOLUTIONS: [u64; {}] = [",
        table.data.len()
    )?;
    for board in table.data {
        writeln!(f, "    0x{:x},", board.to_u64())?;
    }
    writeln!(f, "];")?;
    Ok(())
}

fn write_between(target_dir: &Path) -> Result<(), Box<dyn Error>> {
    let between = chess_lookup_generator::between();
    let mut table = BufWriter::new(File::create(target_dir.join("between.rs"))?);

    writeln!(table, "pub(super) static SOLUTIONS: [[u64; 64]; 64] = [")?;
    for (i, boards) in between.chunks_exact(64).enumerate() {
        writeln!(table, "    [")?;
        for (j, &board) in boards.iter().enumerate() {
            writeln!(table, "        0x{:x},", board.to_u64())?;
            if board.any() {
                let i = Pos::from_u8(i as u8).unwrap();
                let j = Pos::from_u8(j as u8).unwrap();
                eprintln!("{i:?} {j:?}");
                eprintln!("{board:?}");
            }
        }
        writeln!(table, "    ],")?;
    }
    writeln!(table, "];")?;

    Ok(())
}

fn write_line(target_dir: &Path) -> Result<(), Box<dyn Error>> {
    let between = chess_lookup_generator::line();
    let mut table = BufWriter::new(File::create(target_dir.join("line.rs"))?);

    writeln!(table, "pub(super) static SOLUTIONS: [[u64; 64]; 64] = [")?;
    for (i, boards) in between.chunks_exact(64).enumerate() {
        writeln!(table, "    [")?;
        for (j, &board) in boards.iter().enumerate() {
            writeln!(table, "        0x{:x},", board.to_u64())?;
            if board.any() {
                let i = Pos::from_u8(i as u8).unwrap();
                let j = Pos::from_u8(j as u8).unwrap();
                eprintln!("{i:?} {j:?}");
                eprintln!("{board:?}");
            }
        }
        writeln!(table, "    ],")?;
    }
    writeln!(table, "];")?;

    Ok(())
}
