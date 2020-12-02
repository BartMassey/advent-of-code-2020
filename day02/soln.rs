// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 2.  
//! Bart Massey 2020

use aoc::*;

fn read_input() -> Vec<(u64, u64, char, String)> {
    let mut result = Vec::new();
    for line in input_lines() {
        let mut words = line.split(' ');
        let mut digits = words.next().unwrap().split('-');
        let min = digits.next().unwrap().parse().unwrap();
        let max = digits.next().unwrap().parse().unwrap();
        let ch = words.next().unwrap().chars().next().unwrap();
        let password = words.next().unwrap().to_string();
        result.push((min, max, ch, password));
    }
    result
}

pub fn main() {
    let input = read_input();
    let nok = match get_part() {
        Part1 => input
            .iter()
            .filter(|&(min, max, ch, password)| {
                let nch = password.chars().filter(|c| c == ch).count();
                *min <= nch as u64 && nch as u64 <= *max
            })
            .count(),
        Part2 => input
            .iter()
            .filter(|&(p1, p2, ch, password)| {
                let pchars: Vec<char> = password.chars().collect();
                let ok1 = pchars[(*p1 - 1) as usize] == *ch;
                let ok2 = pchars[(*p2 - 1) as usize] == *ch;
                ok1 && !ok2 || ok2 && !ok1
            })
            .count(),
    };
    println!("{}", nok);
}
