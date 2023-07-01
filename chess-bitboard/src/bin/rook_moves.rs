use chess_bitboard::{BitBoard, File, Pos, Rank};
use rand::prelude::*;

struct Blockers {
    puzzle: BitBoard,
    solution: BitBoard,
}

fn main() {
    let mut all_blockers = Vec::new();

    let mut data = vec![BitBoard::empty(); 64 * (1 << 12)];
    let mut rays = vec![BitBoard::empty(); 64 * (1 << 12)];

    let mut current_offset = 0;

    for pos in Pos::all() {
        let mut rook_moves =
            (BitBoard::from(pos.rank()) | BitBoard::from(pos.file())) - BitBoard::from(pos);

        all_blockers.clear();

        let rook_rays = rook_moves;

        if pos.rank() != Rank::_1 {
            rook_moves -= BitBoard::from(Rank::_1);
        }

        if pos.rank() != Rank::_8 {
            rook_moves -= BitBoard::from(Rank::_8);
        }

        if pos.file() != File::A {
            rook_moves -= BitBoard::from(File::A);
        }

        if pos.file() != File::H {
            rook_moves -= BitBoard::from(File::H);
        }

        for idx in 0..1 << rook_moves.count() {
            let mut blockers = BitBoard::empty();
            for blocker in BitBoard::from(idx) {
                let blocker = blocker as usize;
                let blocker = rook_moves.iter().nth(blocker).unwrap();
                blockers.set(blocker);
            }

            assert_eq!(blockers & rook_moves, blockers);

            let mut up = BitBoard::from_pos(pos);
            let mut down = BitBoard::from_pos(pos);
            let mut left = BitBoard::from_pos(pos);
            let mut right = BitBoard::from_pos(pos);

            let mut solution = BitBoard::empty();

            loop {
                up = up.shift_up();
                down = down.shift_down();
                left = left.shift_left();
                right = right.shift_right();

                let all = up | down | left | right;

                if all.none() {
                    break;
                }

                solution |= up | down | left | right;

                up -= blockers;
                down -= blockers;
                left -= blockers;
                right -= blockers;
            }

            all_blockers.push(Blockers {
                puzzle: blockers,
                solution,
            });
        }

        assert_eq!(all_blockers.len(), 1 << rook_moves.count());
        let all_blockers = &mut all_blockers[..];

        let mut new_offset = current_offset;

        'next_offset: for offset in 0..current_offset {
            for (j, blocker) in all_blockers.iter().enumerate() {
                if (rays[offset + j] & rook_rays).any() {
                    continue 'next_offset;
                }
            }

            dbg!();
            new_offset = offset;
            break;
        }

        let mut rng = rand::thread_rng();
        let mut rng = SmallRng::seed_from_u64(0xDEADBEEF12345678);
        let mut best = 0;

        assert!(data.len().is_power_of_two());

        let old_data = data.clone();
        let old_rays = rays.clone();
        let bits = 12;

        dbg!(new_offset);
        dbg!(current_offset);

        assert!(data.len().trailing_zeros() >= bits);

        eprintln!("{pos:?}");
        // eprintln!("{rook_moves:?}");
        'magic: loop {
            let magic = rng.gen::<u64>() & rng.gen::<u64>() & rng.gen::<u64>();

            if magic.wrapping_mul(rook_moves.to_u64()).count_ones() < 6 {
                continue;
            }

            let shift = (all_blockers.len() as u64).leading_zeros() + 1;
            // let offset = rng.gen_range(0..data.len() as u64 / 2);
            let offset = new_offset;

            data.clone_from(&old_data);
            rays.clone_from(&old_rays);

            for (i, p) in all_blockers.iter_mut().enumerate() {
                let index = p.puzzle.to_u64().wrapping_mul(magic) >> shift;
                let index = index as usize + offset;
                let board = &mut data[index];

                if board.none() || *board == p.solution {
                    *board = p.solution;
                    rays[index] |= rook_rays;
                } else {
                    best = best.max(i);
                    if i == best && best != 0 {
                        eprintln!(
                            "{:5.1}% ({i})",
                            i as f64 / all_blockers.len() as f64 * 100.0
                        );
                    }
                    continue 'magic;
                }
            }

            eprintln!("magic = {magic}, shift {shift}, offset = {offset}");

            // panic!()
            break;
        }

        current_offset = (new_offset + all_blockers.len()).max(current_offset);

        // panic!()
    }

    eprintln!("{current_offset}")
}
