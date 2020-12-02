// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 2.  
//! Bart Massey 2020

use aoc::*;

/// Parse of one line from the input.
struct Input {
    /// Lower element of "range".
    lower: usize,
    /// Upper element of "range".
    upper: usize,
    /// Character to consider.
    ch: char,
    /// Password to check.
    password: String,
}

/// Read `Input` records from `stdin`.
fn read_input() -> Vec<Input> {
    let pat = Reparse::new(r"(\d+)-(\d+) (.): ([a-z]+)");
    input_lines()
        .map(|line| {
            let matches = pat.parse(&line);
            let lower = matches.get(1);
            let upper = matches.get(2);
            let ch = matches.get(3);
            let password = matches.get(4);
            Input {
                lower,
                upper,
                ch,
                password,
            }
        })
        .collect()
}

fn part1_filter(r: &Input) -> bool {
    // Count the number of occurrences of `r.ch` in pasword.
    let nch = r.password.chars().filter(|&c| c == r.ch).count();

    // Make sure the count is in-bounds.
    r.lower <= nch && nch <= r.upper
}

fn part2_filter(r: &Input) -> bool {
    // Collect the chars of the password for individual
    // indexing. XXX Yes, this is slow, but it would be a
    // pain to sample the iterator directly.
    let pchars: Vec<char> = r.password.chars().collect();

    // Collect the lower and upper matches.
    let ok_lower = pchars[r.lower - 1] == r.ch;
    let ok_upper = pchars[r.upper - 1] == r.ch;

    // Is exactly one of the matches true?
    ok_lower ^ ok_upper
}

pub fn main() {
    let input = read_input();

    // Pick the appropriate filter.
    let part_filter = match get_part() {
        Part1 => part1_filter,
        Part2 => part2_filter,
    };

    // Count up legal passwords and print the result.
    let nok = input.into_iter().filter(part_filter).count();
    println!("{}", nok);
}
