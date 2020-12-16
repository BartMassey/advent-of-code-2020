// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 7.  
//! Bart Massey 2020

use aoc::*;
use aoc_reparse::*;

use std::collections::HashMap;

type BagRules = HashMap<String, Vec<(usize, String)>>;

fn get_rules() -> BagRules {
    let mut result = HashMap::new();
    let contains_re = Reparse::new(r"^(.*) bags contain (.*)\.");
    let contents_re = Reparse::new(r"^([1-9][0-9]*) (.*) bags?$");
    for line in input_lines() {
        let parsed = contains_re.parse(&line).unwrap();
        let target: String = parsed.get(1);
        let contents: String = parsed.get(2);
        let mut cvec = Vec::new();
        if contents != "no other bags" {
            for baggage in contents.split(", ") {
                let parsed = contents_re.parse(baggage).unwrap();
                let n = parsed.get(1);
                let bag = parsed.get(2);
                cvec.push((n, bag));
            }
        }
        result.insert(target, cvec);
    }
    result
}

fn ok(rules: &BagRules, root: &str, target: &str) -> bool {
    root == target
        || rules[root].iter().any(|(_, bag)| ok(rules, bag, target))
}

fn contained(rules: &BagRules, root: &str) -> usize {
    rules[root]
        .iter()
        .map(|(count, bag)| count * (1 + contained(rules, bag)))
        .sum()
}

fn main() {
    let rules = get_rules();
    let target = "shiny gold";
    match get_part() {
        Part1 => {
            let nstart = rules
                .keys()
                .filter(|&bag| bag != target)
                .filter(|bag| ok(&rules, bag, target))
                .count();
            println!("{}", nstart);
        }
        Part2 => {
            let ncontained = contained(&rules, target);
            println!("{}", ncontained);
        }
    }
}
