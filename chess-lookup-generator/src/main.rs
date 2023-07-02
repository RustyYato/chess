use std::{error::Error, fs::File, io::BufWriter, io::Write, path::Path};

use chess_bitboard::Pos;
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

    assert!(target_dir.exists());

    write_rook_rays(target_dir)?;
    write_bishop_rays(target_dir)?;
    write_bishop_moves(target_dir)?;
    write_rook_moves(target_dir)?;

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