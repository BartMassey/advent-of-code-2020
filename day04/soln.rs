// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 4.  
//! Bart Massey 2020

use std::collections::HashMap;

use aoc::*;

const EYE_COLORS: &[&str] = &[
    "amb", "blu", "brn", "gry", "grn", "hzl", "oth"
];

const KEYS: &[(&str, fn(&str)->bool)] = &[
    ("byr", |v: &str| {
        match v.parse::<u64>() {
            Ok(v) => v >= 1920 && v <= 2002,
            Err(_) => false,
        }
    }),
    ("iyr", |v: &str| {
        match v.parse::<u64>() {
            Ok(v) => v >= 2010 && v <= 2020,
            Err(_) => false,
        }
    }),
    ("eyr", |v: &str| {
        match v.parse::<u64>() {
            Ok(v) => v >= 2020 && v <= 2030,
            Err(_) => false,
        }
    }),
    ("hgt", |v: &str| {
        let nv = v.len();
        if nv < 3 {
            return false;
        }
        let (num, units) = v.split_at(nv - 2);
        let num = match num.parse::<u64>() {
            Ok(v) => v,
            Err(_) => return false,
        };
        match units {
            "cm" => num >= 150 && num <= 193,
            "in" => num >= 59 && num <= 76,
            _ => false,
        }
    }),
    ("hcl", |v: &str| {
        let chs: Vec<char> = v.chars().collect();
        if chs.len() != 7 || chs[0] != '#' {
            return false;
        }
        chs[1..].into_iter().all(|c| c.is_digit(16))
    }),
    ("ecl", |v: &str| {
        EYE_COLORS.iter().any(|&c| v == c)
    }),
    ("pid", |v: &str| {
        let chs: Vec<char> = v.chars().collect();
        if chs.len() != 9 {
            return false;
        }
        chs.into_iter().all(|c| c.is_digit(10))
    }),
    //"cid",
];

fn get_passports() -> Vec<HashMap<String, String>> {
    let input: String = input_lines()
        .map(|s| {
            (s.replace(" ", "\n") + "\n").to_string()
        })
        .collect();
    let pars: Vec<String> = input.split("\n\n").map(str::to_owned).collect();
    pars.into_iter()
        .map(|p| {
            p.split("\n")
                .filter(|&s| !s.is_empty())
                .map(|f| {
                    let fields: Vec<&str> = f
                        .split(":")
                        .collect();
                    (fields[0].to_owned(), fields[1].to_owned())
                })
                .collect()
        })
        .collect()
}

pub fn main() {
    let passports = get_passports();
    match get_part() {
        Part1 => {
            let nvalid = passports.into_iter().filter(|p| {
                KEYS.iter().all(|&(k, _)| p.contains_key(k))
            }).count();
            println!("{}", nvalid);
        },
        Part2 => {
            let nvalid = passports.into_iter().filter(|p| {
                //eprintln!();
                KEYS.iter().all(|&(k, validate)| {
                    if let Some(v) = p.get(k) {
                        let result = validate(v);
                        //eprintln!("{}:{} {:?}", k, v, result);
                        result
                    } else {
                        //eprintln!("{}: no key", k);
                        false
                    }
                })
            }).count();
            println!("{}", nvalid);
        },
    }
}
