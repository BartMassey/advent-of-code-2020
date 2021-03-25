// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 17.  
//! Bart Massey 2020

use aoc::*;

use std::collections::HashSet;

type Board<const D: usize> = HashSet<[isize; D]>;

fn read_initial<const D: usize>() -> Board<D> {
    let mut initial = HashSet::new();
    for (row, l) in input_lines().enumerate() {
        for (col, c) in l.chars().enumerate() {
            let mut template = [0; D];
            match c {
                '#' => {
                    template[0] = row as isize;
                    template[1] = col as isize;
                    initial.insert(template);
                }
                '.' => (),
                c => panic!("unexpected char {} in input", c),
            }
        }
    }
    initial
}

/// Returns a vector of offsets for the problem
/// neighborhood.  Constructed so that the last offset is
/// all-zeros. Could be an iterator, but constructing just
/// once might be faster.
fn offsets<const D: usize>(
    i: usize,
    mut template: [isize; D],
) -> Vec<[isize; D]> {
    if i == D {
        return vec![template];
    }

    let mut acc = Vec::with_capacity(usize::pow(3, (D - i) as u32));
    for &j in &[-1, 1, 0] {
        template[i] = j;
        let xo = offsets(i + 1, template);
        acc.extend(xo);
    }
    acc
}

fn iter_life<const D: usize>(mut state: Board<D>, count: usize) -> Board<D> {
    let mut off = offsets(0, [0; D]);
    // Remove all-zeros "neighbor".
    let _ = off.pop();
    for _ in 0..count {
        let mut next = HashSet::new();
        let mut empties = HashSet::new();

        for p in &state {
            let mut neighbors = 0;
            for dp in &off {
                let mut xp = *p;
                for i in 0..D {
                    xp[i] += dp[i];
                }
                if state.contains(&xp) {
                    neighbors += 1;
                } else {
                    empties.insert(xp);
                }
            }

            if neighbors == 2 || neighbors == 3 {
                next.insert(*p);
            }
        }

        for p in &empties {
            let mut neighbors = 0;
            for dp in &off {
                let mut xp = *p;
                for i in 0..D {
                    xp[i] += dp[i];
                }
                if state.contains(&xp) {
                    neighbors += 1;
                }
            }

            if neighbors == 3 {
                next.insert(*p);
            }
        }

        state = next;
    }
    state
}

fn solve<const D: usize>() -> usize {
    let initially: Board<D> = read_initial();
    let finally = iter_life(initially, 6);
    finally.len()
}

fn main() {
    let n = match get_part() {
        Part1 => solve::<3>(),
        Part2 => solve::<4>(),
    };
    println!("{}", n);
}
