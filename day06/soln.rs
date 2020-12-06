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
        .split("\n\n")
        .map(|group| {
            group
                .split('\n')
                .map(|person| person.chars().collect())
                .collect()
        })
        .collect()
}

fn compute(
    sets: Vec<Vec<HashSet<char>>>,
    combine: fn(HashSet<char>, HashSet<char>) -> HashSet<char>,
) -> usize {
    sets.into_iter()
        .map(|mut group| {
            let last = match group.pop() {
                None => return 0,
                Some(p) => p,
            };
            group.into_iter().fold(last, combine).len()
        })
        .sum()
}

pub fn main() {
    let sets = read_sets();
    let count: usize = match get_part() {
        Part1 => compute(sets, |p, u| p.union(&u).cloned().collect()),
        Part2 => {
            compute(sets, |p, u| p.intersection(&u).cloned().collect())
        }
    };
    println!("{}", count);
}
