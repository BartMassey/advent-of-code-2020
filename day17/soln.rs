// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 17.  
//! Bart Massey 2020

use aoc::*;

use std::collections::HashSet;

type Board3 = HashSet<[isize;3]>;
type Board4 = HashSet<[isize;4]>;

fn read_initial3() -> Board3 {
    let mut initial = HashSet::new();
    for (row, l) in input_lines().enumerate() {
        for (col, c) in l.chars().enumerate() {
            match c {
                '.' => (),
                '#' => {initial.insert([row as isize, col as isize, 0]);},
                c => panic!("unexpected char {} in input", c),
            }
        }
    }
    initial
}

fn read_initial4() -> Board4 {
    let mut initial = HashSet::new();
    for (row, l) in input_lines().enumerate() {
        for (col, c) in l.chars().enumerate() {
            match c {
                '.' => (),
                '#' => {initial.insert([row as isize, col as isize, 0, 0]);},
                c => panic!("unexpected char {} in input", c),
            }
        }
    }
    initial
}

fn iter_life3(state: &mut Board3, count: usize) {
    for _ in 0..count {
        let mut next = HashSet::new();

        let mut empties = HashSet::new();
        for &[r, c, p] in state.iter() {
            let mut neighbors = 0;
            for dr in -1..=1 {
                for dc in -1..=1 {
                    for dp in -1..=1 {
                        if dr == 0 && dc == 0 && dp == 0 {
                            continue;
                        }
                        let nb = [r + dr, c + dc, p + dp];
                        if state.contains(&nb) {
                            neighbors += 1;
                        } else {
                            empties.insert(nb);
                        }
                    }
                }
            }
            // println!("1: {}, {}, {} → {}", r, c, p, neighbors);
            if neighbors == 2 || neighbors == 3 {
                next.insert([r, c, p]);
            }
        }

        for &[r, c, p] in empties.iter() {
            let mut neighbors = 0;
            for dr in -1..=1 {
                for dc in -1..=1 {
                    for dp in -1..=1 {
                        if dr == 0 && dc == 0 && dp == 0 {
                            continue;
                        }
                        let nb = [r + dr, c + dc, p + dp];
                        if state.contains(&nb) {
                            neighbors += 1;
                        }
                    }
                }
            }
            // println!("2: {}, {}, {} → {}", r, c, p, neighbors);
            if neighbors == 3 {
                next.insert([r, c, p]);
            }
        }

        *state = next;
    }
}

fn iter_life4(state: &mut Board4, count: usize) {
    for _ in 0..count {
        let mut next = HashSet::new();

        let mut empties = HashSet::new();
        for &[r, c, p, h] in state.iter() {
            let mut neighbors = 0;
            for dr in -1..=1 {
                for dc in -1..=1 {
                    for dp in -1..=1 {
                        for dh in -1..=1 {
                            if dr == 0 && dc == 0 && dp == 0 && dh == 0 {
                                continue;
                            }
                            let nb = [r + dr, c + dc, p + dp, h + dh];
                            if state.contains(&nb) {
                                neighbors += 1;
                            } else {
                                empties.insert(nb);
                            }
                        }
                    }
                }
            }
            if neighbors == 2 || neighbors == 3 {
                next.insert([r, c, p, h]);
            }
        }

        for &[r, c, p, h] in empties.iter() {
            let mut neighbors = 0;
            for dr in -1..=1 {
                for dc in -1..=1 {
                    for dp in -1..=1 {
                        for dh in -1..=1 {
                            if dr == 0 && dc == 0 && dp == 0 && dh == 0 {
                                continue;
                            }
                            let nb = [r + dr, c + dc, p + dp, h + dh];
                            if state.contains(&nb) {
                                neighbors += 1;
                            }
                        }
                    }
                }
            }
            if neighbors == 3 {
                next.insert([r, c, p, h]);
            }
        }

        *state = next;
    }
}

fn main() {
    match get_part() {
        Part1 => {
            let mut initial = read_initial3();
            // println!("{:#?}", initial);
            iter_life3(&mut initial, 6);
            println!("{}", initial.len());
        }
        Part2 => {
            let mut initial = read_initial4();
            iter_life4(&mut initial, 6);
            println!("{}", initial.len());
        }
    }
}
