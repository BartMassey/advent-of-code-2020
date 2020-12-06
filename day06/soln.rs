// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 6.  
//! Bart Massey 2020

use std::collections::HashSet;

use aoc::*;

fn read_sets() -> Vec<Vec<HashSet<char>>> {
    use std::io::Read;
    
    let mut stdin = std::io::stdin();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    input
        .split("\n\n").
        map(|group| {
            group
                .split("\n")
                .map(|person| {
                    person.chars().collect()
                })
                .collect()
        })
        .collect()
}

pub fn main() {
    let sets = read_sets();
    match get_part() {
        Part1 => {
            let count: usize = sets
                .into_iter()
                .map(|group| {
                    group
                        .into_iter()
                        .fold(HashSet::new(), |u, p| {
                            p.union(&u).cloned().collect()
                        })
                        .len()
                })
                .sum();
            println!("{}", count);
        },
        Part2 => {
            let count: usize = sets
                .into_iter()
                .map(|mut group| {
                    let last = match group.pop() {
                        None => return 0,
                        Some(p) => p,
                    };
                    group
                        .into_iter()
                        .fold(last, |u, p| {
                            p.intersection(&u).cloned().collect()
                        })
                        .len()
                })
                .sum();
            println!("{}", count);
        },
    }
}
