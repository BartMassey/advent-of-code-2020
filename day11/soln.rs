// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 11.  
//! Bart Massey 2020

use aoc::*;

fn read_seats() -> Vec<Vec<char>> {
    input_lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn iterate_near(v: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result = v.clone();
    let rows = v.len();
    let cols = v[0].len();
    let bump_down = |v| if v > 0 { v - 1 } else { v };
    let bump_up = |v, lim| if v < lim { v + 1 } else { v };
    for r in 0..rows {
        for c in 0..cols {
            let seat = v[r][c];
            if seat == '.' {
                continue;
            }
            let top = bump_down(r);
            let bot = bump_up(r, rows - 1);
            let left = bump_down(c);
            let right = bump_up(c, cols - 1);

            let mut neighbors = 0;
            //println!("{}..{} {}..{}", top, bot, left, right);
            for dr in top..=bot {
                for dc in left..=right {
                    if v[dr][dc] == '#' {
                        neighbors += 1;
                    }
                }
            }
            if seat == '#' {
                neighbors -= 1;
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

fn iterate_far(v: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result = v.clone();
    let rows = v.len();
    let cols = v[0].len();
    for r in 0..rows {
        for c in 0..cols {
            let seat = v[r][c];
            if seat == '.' {
                continue;
            }
            let trace = |dr, dc| {
                let mut r0 = r;
                let mut c0 = c;
                let step = move || {
                    let nr0 = r0 as isize + dr;
                    let nc0 = c0 as isize + dc;
                    if nr0 < 0 || nr0 >= rows as isize {
                        return None;
                    }
                    if nc0 < 0 || nc0 >= cols as isize {
                        return None;
                    }
                    r0 = nr0 as usize;
                    c0 = nc0 as usize;
                    Some((r0, c0))
                };
                std::iter::from_fn(step)
            };

            let mut neighbors = 0;
            for dr in -1..=1 {
                for dc in -1..=1 {
                    if dr == 0 && dc == 0 {
                        continue;
                    }
                    for (r0, c0) in trace(dr, dc) {
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

fn count_seats(input: &Vec<Vec<char>>) -> usize {
    input
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&c| c == '#')
        .count()
}

#[allow(dead_code)]
fn show_map(map: &Vec<Vec<char>>) {
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
