// Copyright © 2020 Bart Massey
// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Regex-based line parsing for Advent of Code 2020 solutions.

use std::str::FromStr;
use std::fmt::Debug;

pub use regex::{Regex, Captures};

pub struct Reparse(Regex);

pub struct Rematch<'a>(Captures<'a>);

impl Reparse {
    pub fn new(pat: &str) -> Self {
        Reparse(Regex::new(pat).unwrap())
    }

    pub fn parse<'a>(&self, line: &'a str) -> Rematch<'a> {
        Rematch(self.0.captures(line).unwrap())
    }
}

impl<'a> Rematch<'a> {
    pub fn get<T>(&'a self, index: usize) -> T
        where T: FromStr, <T as FromStr>::Err: Debug,
    {
        self.0.get(index).unwrap().as_str().parse().unwrap()
    }
}
