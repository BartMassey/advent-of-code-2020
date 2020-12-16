// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 11.  
//! Bart Massey 2020

use aoc::*;
use aoc_geom::*;

fn read_seats() -> Vec<Vec<char>> {
    input_lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn iterate_near(v: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut result = v.to_owned();
    let rows = v.len();
    let cols = v[0].len();
    let grid = GridBox::new(rows, cols);
    for r in 0..rows {
        for c in 0..cols {
            let seat = v[r][c];
            if seat == '.' {
                continue;
            }

            let mut neighbors = 0;
            for (dr, dc) in grid.neighbors((r, c), 1) {
                //println!("{} {}", dr, dc);
                if v[dr][dc] == '#' {
                    neighbors += 1;
                }
            }
            //println!("{} {} {}", r, c, neighbors);

            if seat == 'L' && neighbors == 0 {
                result[r][c] = '#';
            }
            if seat == '#' && neighbors >= 4 {
                result[r][c] = 'L';
            }
        }
    }
    result
}

fn iterate_far(v: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut result = v.to_owned();
    let rows = v.len();
    let cols = v[0].len();
    let grid = GridBox::new(rows, cols);
    for r in 0..rows {
        for c in 0..cols {
            let seat = v[r][c];
            if seat == '.' {
                continue;
            }

            let mut neighbors = 0;
            for (dr, dc) in neighbors8::<i64, i64>(1) {
                for (r0, c0) in grid.beam((r, c), (dr, dc)) {
                    let target = v[r0][c0];
                    if target == '#' {
                        neighbors += 1;
                        break;
                    }
                    if target == 'L' {
                        break;
                    }
                }
            }
            //println!("{} {} {}", r, c, neighbors);

            if seat == 'L' && neighbors == 0 {
                result[r][c] = '#';
            }
            if seat == '#' && neighbors >= 5 {
                result[r][c] = 'L';
            }
        }
    }
    result
}

fn count_seats(input: &[Vec<char>]) -> usize {
    input
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&c| c == '#')
        .count()
}

#[allow(dead_code)]
fn show_map(map: &[Vec<char>]) {
    for r in map {
        for c in r {
            print!("{}", c);
        }
        println!();
    }
}

fn main() {
    let mut input = read_seats();
    // show_map(&input);
    match get_part() {
        Part1 => {
            let mut next = iterate_near(&input);
            while next != input {
                // println!();
                // show_map(&next);
                input = next;
                next = iterate_near(&input);
            }
            let occ = count_seats(&input);
            println!("{}", occ);
        }
        Part2 => {
            let mut next = iterate_far(&input);
            while next != input {
                // println!();
                // show_map(&next);
                input = next;
                next = iterate_far(&input);
            }
            let occ = count_seats(&input);
            println!("{}", occ);
        }
    }
}
