// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 2.  
//! Bart Massey 2020

use aoc::*;

fn read_input() -> Vec<(usize, usize, char, String)> {
    let pat = Reparse::new(r"(\d+)-(\d+) (.): ([a-z]+)");
    input_lines()
        .map(|line| {
            let matches = pat.parse(&line);
            let min = matches.get(1);
            let max = matches.get(2);
            let ch = matches.get(3);
            let password = matches.get(4);
            (min, max, ch, password)
        })
        .collect()
}

pub fn main() {
    let input = read_input();
    let nok = match get_part() {
        Part1 => input
            .iter()
            .filter(|&(min, max, ch, password)| {
                let nch = password.chars().filter(|c| c == ch).count();
                *min <= nch && nch <= *max
            })
            .count(),
        Part2 => input
            .iter()
            .filter(|&(p1, p2, ch, password)| {
                let pchars: Vec<char> = password.chars().collect();
                let ok1 = pchars[*p1 - 1] == *ch;
                let ok2 = pchars[*p2 - 1] == *ch;
                ok1 && !ok2 || ok2 && !ok1
            })
            .count(),
    };
    println!("{}", nok);
}
