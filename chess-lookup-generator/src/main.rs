use std::{error::Error, fs::File, io::BufWriter, io::Write, path::Path};

use chess_bitboard::Pos;

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

    Ok(())
}

fn write_rook_rays(target_dir: &Path) -> Result<(), Box<dyn Error>> {
    let mut rook_rays = BufWriter::new(File::create(target_dir.join("rook_rays.rs"))?);

    writeln!(rook_rays, "pub(super) static RAYS: [u64; 64] = [")?;
    for pos in Pos::all() {
        let rays = chess_lookup_generator::rook_rays(pos);
        writeln!(rook_rays, "    0x{:x},", rays.to_u64())?;
    }
    writeln!(rook_rays, "];")?;
    Ok(())
}

fn write_bishop_rays(target_dir: &Path) -> Result<(), Box<dyn Error>> {
    let mut rook_rays = BufWriter::new(File::create(target_dir.join("bishop_rays.rs"))?);

    writeln!(rook_rays, "pub(super) static RAYS: [u64; 64] = [")?;
    for pos in Pos::all() {
        let rays = chess_lookup_generator::bishop_rays(pos);
        writeln!(rook_rays, "    0x{:x},", rays.to_u64())?;
    }
    writeln!(rook_rays, "];")?;
    Ok(())
}
