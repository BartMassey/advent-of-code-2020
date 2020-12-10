// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 10.  
//! Bart Massey 2020

use aoc::*;

fn read_adapters() -> Vec<u64> {
    let mut v: Vec<u64> = input_lines()
        .map(|s| s.parse().unwrap())
        .collect();
    v.sort();
    v
}

fn chain_all(adapters: &[u64]) -> Option<[usize;4]> {
    let mut deltas = [0, 0, 0, 1];
    if adapters[0] > 3 {
        return None;
    }
    deltas[adapters[0] as usize] += 1;
    for (&a1, &a2) in adapters.iter().zip(adapters[1..].iter()) {
        let d = a2 - a1;
        if d > 3 {
            return None;
        }
        deltas[d as usize] += 1;
    }
    Some(deltas)
}

fn combinations(adapters: &[u64]) -> u64 {
    let nadapters = adapters.len();
    let laptop = adapters[nadapters - 1] as usize;
    let mut have: Vec<bool> = Vec::with_capacity(laptop + 1);
    have.resize_with(laptop + 1, Default::default);
    for &a in adapters {
        have[a as usize] = true;
    }

    let mut c = vec![1, 0, 0];
    //println!("{} {}", 0, c[0]);
    if have[1] {
            c[1] = 1;
    }
    //println!("{} {}", 1, c[1]);
    if have[2] {
            c[2] = 1 + c[1];
    }
    //println!("{} {}", 2, c[2]);
    for i in 3..=laptop {
        let n = if have[i] {
            c[i - 3 ..= i - 1].iter().cloned().sum()
        } else {
            0
        };
        //println!("{} {}", i, n);
        c.push(n);
    }
    c[laptop]
}

fn main() {
    let adapters = read_adapters();
    match get_part() {
        Part1 => {
            let deltas = chain_all(&adapters).unwrap();
            println!("{}", deltas[1] * deltas[3]);
        }
        Part2 => {
            let combos = combinations(&adapters);
            println!("{}", combos);
        }
    }
}
