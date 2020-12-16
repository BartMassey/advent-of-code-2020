// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 15.  
//! Bart Massey 2020

use aoc::*;

fn read_starts() -> Vec<u64> {
    input_lines()
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

fn game(starts: Vec<u64>, turns: usize) -> u64 {
    let mut si = starts.iter();
    let nstarts = starts.len();
    let mut mem = vec![0; turns];
    for (t, &s) in (&mut si).take(nstarts - 1).enumerate() {
        // println!("*{}: {}", t, s);
        mem[s as usize] = t + 1;
    }
    let mut last = *si.next().unwrap();
    for t in nstarts .. turns {
        // println!("{}: {}", t, last);
        let cur = match mem[last as usize] {
            0 => 0,
            t_prev => (t - t_prev) as u64,
        };
        mem[last as usize] = t;
        last = cur;
    }
    //println!("{}", mem.len());
    last
}

fn main() {
    let starts = read_starts();
    // println!("{:#?}", starts);
    match get_part() {
        Part1 => {
            let last = game(starts, 2020);
            println!("{}", last);
        }
        Part2 => {
            let last = game(starts, 30_000_000);
            println!("{}", last);
        }
    }
}
