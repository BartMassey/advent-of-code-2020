// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

#![feature(min_const_generics)]

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

/// Generate the offsets for the neighborhood of a
/// coordinate. Constructed such that the last delta is
/// all-zeros.
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

fn iter_life<const D: usize>(state: &mut Board<D>, count: usize) {
    let mut off = offsets(0, [0; D]);
    // Discard all-zeros offset.
    let _ = off.pop();

    let mut next = HashSet::new();
    let mut pages = [&mut *state, &mut next];
    let mut cands = HashSet::new();

    for _ in 0..count {
        let [cur, next] = pages;
        next.clear();

        cands.clone_from(cur);
        for p in cur.iter() {
            for &dp in &off {
                let mut p = *p;
                for i in 0..D {
                    p[i] += dp[i];
                }
                cands.insert(p);
            }
        }

        for p in cands.iter() {
            let mut neighbors = 0;
            for &dp in &off {
                let mut p = *p;
                for i in 0..D {
                    p[i] += dp[i];
                }
                if cur.contains(&p) {
                    neighbors += 1;
                }
            }

            if (neighbors == 2 && cur.contains(p)) || neighbors == 3 {
                next.insert(*p);
            }
        }

        pages = [next, cur];
    }
    let [cur, _] = pages;
    // XXX Cannot figure out how to get the borrow
    // checker to let me just move `*cur`.
    *state = cur.clone();
}

fn main() {
    match get_part() {
        Part1 => {
            let mut initial: Board<3> = read_initial();
            iter_life(&mut initial, 6);
            println!("{}", initial.len());
        }
        Part2 => {
            let mut initial: Board<4> = read_initial();
            iter_life(&mut initial, 6);
            println!("{}", initial.len());
        }
    }
}
