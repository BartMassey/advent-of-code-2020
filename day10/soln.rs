// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 10.  
//! Bart Massey 2020

use std::collections::HashSet;

use aoc::*;

fn read_adapters() -> HashSet<u64> {
    input_lines()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn chain_all(
    adapters: &HashSet<u64>,
    cur: u64,
    laptop: u64,
) -> Option<[usize;4]> {
    if adapters.is_empty() {
        if cur == laptop {
            //println!("solved");
            return Some([0, 0, 0, 1]);
        }
        //println!("empty {} {}", cur, laptop);
        return None;
    }

    for diff in 1..=3 {
        let cur = cur + diff;
        if adapters.contains(&cur) {
            //println!("trying {}", cur);
            let mut ad0 = adapters.clone();
            ad0.remove(&cur);
            if let Some(mut deltas) = chain_all(&ad0, cur, laptop) {
                deltas[diff as usize] += 1;
                return Some(deltas);
            }
        }
    }
    None
}

fn combinations(adapters: &HashSet<u64>, laptop: u64) -> u64 {
    let mut c = vec![1, 0, 0];
    //println!("{} {}", 0, c[0]);
    if adapters.contains(&1) {
            c[1] = 1;
    }
    //println!("{} {}", 1, c[1]);
    if adapters.contains(&2) {
            c[2] = 1 + c[1];
    }
    //println!("{} {}", 2, c[2]);
    for i in 3..=laptop {
        let n = if adapters.contains(&i) {
            let iu = i as usize;
            c[iu - 3 ..= iu - 1].iter().cloned().sum()
        } else {
            0
        };
        //println!("{} {}", i, n);
        c.push(n);
    }
    c[laptop as usize]
}

fn main() {
    let adapters = read_adapters();
    let laptop = adapters.iter().cloned().max().unwrap();
    match get_part() {
        Part1 => {
            let deltas = chain_all(&adapters, 0, laptop).unwrap();
            println!("{}", deltas[1] * deltas[3]);
        }
        Part2 => {
            let combos = combinations(&adapters, laptop);
            println!("{}", combos);
        }
    }
}
