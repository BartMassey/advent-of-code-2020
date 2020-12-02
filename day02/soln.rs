// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 2.  
//! Bart Massey 2020

use aoc::*;

struct Record {
    min: usize,
    max: usize,
    ch: char,
    password: String,
}

fn read_input() -> Vec<Record> {
    let pat = Reparse::new(r"(\d+)-(\d+) (.): ([a-z]+)");
    input_lines()
        .map(|line| {
            let matches = pat.parse(&line);
            let min = matches.get(1);
            let max = matches.get(2);
            let ch = matches.get(3);
            let password = matches.get(4);
            Record { min, max, ch, password }
        })
        .collect()
}

fn part1_filter(r: &Record) -> bool {
    let nch = r.password.chars().filter(|&c| c == r.ch).count();
    r.min <= nch && nch <= r.max
}

fn part2_filter(r: &Record) -> bool {
    let pchars: Vec<char> = r.password.chars().collect();
    let ok1 = pchars[r.min - 1] == r.ch;
    let ok2 = pchars[r.max - 1] == r.ch;
    ok1 ^ ok2
}

pub fn main() {
    let input = read_input();
    let part_filter = match get_part() {
        Part1 => part1_filter,
        Part2 => part2_filter,
    };
    let nok = input
        .into_iter()
        .filter(part_filter)
        .count();
    println!("{}", nok);
}
