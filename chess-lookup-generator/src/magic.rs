use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering::Relaxed},
    Mutex,
};

use chess_bitboard::{BitBoard, Pos};
use rand::{prelude::*, rngs::SmallRng};

struct Blockers {
    puzzle: BitBoard,
    solution: BitBoard,
}

pub struct MagicTableEntry {
    pub mask: BitBoard,
    pub factor: u64,
    pub shift: u32,
    pub offset: usize,
}

pub struct MagicTable {
    pub entries: Vec<MagicTableEntry>,
    pub table: Vec<BitBoard>,
}

pub fn generate_tables<F, S>(get_rays_and_moves: F, solve: S) -> MagicTable
where
    F: Fn(Pos) -> (BitBoard, BitBoard),
    S: Fn(Pos, BitBoard) -> BitBoard,
{
    let mut all_blockers = Vec::new();

    let mut table = vec![BitBoard::empty(); 64 * (1 << 12)];
    let mut rays = vec![BitBoard::empty(); 64 * (1 << 12)];

    let mut entries = Vec::new();

    let mut current_offset = 0;

    for pos in Pos::all() {
        eprintln!("{pos:?}");
        all_blockers.clear();

        let (piece_rays, piece_moves) = get_rays_and_moves(pos);

        for idx in 0..1 << piece_moves.count() {
            let mut blockers = BitBoard::empty();
            for blocker in BitBoard::from(idx) {
                let blocker = blocker as usize;
                let blocker = piece_moves.iter().nth(blocker).unwrap();
                blockers.set(blocker);
            }

            assert_eq!(blockers & piece_moves, blockers);

            all_blockers.push(Blockers {
                puzzle: blockers,
                solution: solve(pos, blockers),
            });
        }

        assert_eq!(all_blockers.len(), 1 << piece_moves.count());
        let all_blockers = &mut all_blockers[..];

        let new_offset = current_offset;
        let best = AtomicUsize::new(0);

        assert!(table.len().is_power_of_two());

        let old_data = table.clone();
        let old_rays = rays.clone();
        let bits = 12;

        assert!(table.len().trailing_zeros() >= bits);

        let finished = AtomicBool::new(false);
        let output = Mutex::new(None);

        std::thread::scope(|s| {
            for _ in 0..23 {
                s.spawn(|| {
                    let mut data = Vec::new();
                    let mut rays = Vec::new();
                    let mut rng = SmallRng::from_seed(rand::random());

                    'magic: loop {
                        let magic = rng.gen::<u64>() & rng.gen::<u64>() & rng.gen::<u64>();

                        if magic.wrapping_mul(piece_moves.to_u64()).count_ones() < 6 {
                            continue;
                        }

                        if finished.load(Relaxed) {
                            break;
                        }

                        let shift = (all_blockers.len() as u64).leading_zeros() + 1;
                        let offset = new_offset;

                        data.clone_from(&old_data);
                        rays.clone_from(&old_rays);

                        for (i, p) in all_blockers.iter().enumerate() {
                            let index = p.puzzle.to_u64().wrapping_mul(magic) >> shift;
                            let index = index as usize + offset;
                            let board = &mut data[index];

                            if board.none() || *board == p.solution {
                                *board = p.solution;
                                rays[index] |= piece_rays;
                            } else {
                                let best = best.fetch_max(i, Relaxed);
                                if best < i {
                                    eprintln!(
                                        "{:5.1}% ({i})",
                                        i as f64 / all_blockers.len() as f64 * 100.0
                                    );
                                }
                                continue 'magic;
                            }
                        }

                        eprintln!("magic = {magic}, shift {shift}, offset = {offset}");

                        if !finished.swap(true, Relaxed) {
                            let output = &mut *output.try_lock().unwrap();
                            *output = Some((magic, shift, offset, data, rays));
                        }

                        break;
                    }
                });
            }
        });

        let (magic, shift, offset, new_table, new_rays) =
            { output }.get_mut().unwrap().take().unwrap();
        table = new_table;
        rays = new_rays;

        entries.push(MagicTableEntry {
            mask: piece_moves,
            factor: magic,
            shift,
            offset,
        });

        current_offset = (new_offset + all_blockers.len()).max(current_offset);
    }

    eprintln!("{current_offset}");

    MagicTable { entries, table }
}
