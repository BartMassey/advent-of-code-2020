// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 5.  
//! Bart Massey 2020

use aoc::*;

fn read_passes() -> Vec<(u64, u64)> {
    fn to_coord(zero: char, one: char, letters: &str) -> u64 {
        let num: String = letters
            .chars()
            .map(|c| {
                if c == zero {
                    '0'
                } else if c == one {
                    '1'
                } else {
                    panic!("unknown letter");
                }
            })
            .collect();
        u64::from_str_radix(&num, 2).unwrap()
    }

    input_lines()
        .map(|line| {
            let row = to_coord('F', 'B', &line[0..7]);
            let col = to_coord('L', 'R', &line[7..]);
            (row, col)
        })
        .collect()
}

pub fn main() {
    let passes = read_passes();
    match get_part() {
        Part1 => {
            let highest = passes
                .into_iter()
                .map(|(r, c)| r * 8 + c)
                .max()
                .unwrap();
            println!("{}", highest);
        },
        Part2 => {
            let mut ids: Vec<u64> = passes
                .into_iter()
                .map(|(r, c)| r * 8 + c)
                .collect();
            ids.sort();
            for i in 0 .. ids.len() - 1 {
                if ids[i] + 2 == ids[i + 1] {
                    println!("{}", ids[i] + 1);
                }
            }
        },
    }
}
