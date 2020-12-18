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

fn iter_life<const D: usize>(state: Board<D>, count: usize) -> Board<D> {
    let mut off = offsets(0, [0; D]);
    // Discard all-zeros offset.
    let _ = off.pop();

    let next = HashSet::new();
    let mut pages = [state, next];
    let mut cands = HashSet::new();

    for _ in 0..count {
        let [cur, mut next] = pages;
        next.clear();

        cands.clone_from(&cur);
        for p in &cur {
            for &dp in &off {
                let mut p = *p;
                for i in 0..D {
                    p[i] += dp[i];
                }
                cands.insert(p);
            }
        }

        for p in &cands {
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
    cur
}

fn solve<const D: usize>() -> usize {
    let initial: Board<D> = read_initial();
    let finally = iter_life(initial, 6);
    finally.len()
}

fn main() {
    let n = match get_part() {
        Part1 => solve::<3>(),
        Part2 => solve::<4>(),
    };
    println!("{}", n);
}
