// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 19.  
//! Bart Massey 2020

use std::collections::HashMap;
use std::io::{stdin, Read};

use aoc_reparse::*;

type Grammar = HashMap<u64, Prod>;
type Message = Vec<char>;

#[derive(Debug, Clone)]
struct Comms {
    grammar: Grammar,
    messages: Vec<Message>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Prod {
    T(char),
    NT {
        left: Vec<u64>,
        right: Option<Vec<u64>>,
    }
}
use Prod::*;


fn read_comms() -> Comms {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut fields = input.split("\n\n");
    let grammar = fields.next().unwrap();
    let messages = fields.next().unwrap();
    assert!(fields.next().is_none());

    let t_re = Reparse::new(r#"^([1-9][0-9]*): "([ab])"$"#);
    let nt_re = Reparse::new(
        r#"^([0-9]+): ([0-9]+( [0-9]+)*)( \| ([0-9]+( [0-9]+)*)+)?$"#);
    let grammar = grammar
        .split('\n')
        .map(|l| {
            if let Some(p) = t_re.parse(l) {
                let lhs = p.get(1);
                let rhs: String = p.get(2);
                (lhs, T(rhs.chars().next().unwrap()))
            } else if let Some(p) = nt_re.parse(l) {
                let lhs = p.get(1);
                let fields = |s: String| s
                    .split(' ')
                    //.inspect(|s| println!("{}", s))
                    .map(|n| n.parse().unwrap())
                    .collect();
                let rhs1 = fields(p.get(2));
                let rhs2 = p.get_opt(5).map(|s| fields(s));
                (
                    lhs,
                    NT {
                        left: rhs1,
                        right: rhs2,
                    },
                )
            } else {
                panic!("failed parse: {}", l);
            }
        })
        .collect();

    let messages = messages
        .trim()
        .split('\n')
        .map(|l| l.chars().collect())
        .collect();

    Comms { grammar, messages }
}

fn has_parse(grammar: &Grammar, message: &[char], start: u64) -> Option<usize> {
    let prod = &grammar[&start];
    let try_parse = |terms: &[u64]| -> Option<usize> {
        let mut mslice = message;
        let mut tot = 0;
        for &term in terms {
            match has_parse(grammar, mslice, term) {
                Some(n) => {
                    mslice = &mslice[n..];
                    tot += n;
                }
                None => return None,
            }
        }
        if start == 0 && !mslice.is_empty() {
            return None;
        }
        Some(tot)
    };
    match prod {
        T(c) => {
            if message.get(0) == Some(c) {
                Some(1)
            } else {
                None
            }
        }
        NT { left, right } => {
            try_parse(left)
                .or_else(|| {
                    right.as_ref().and_then(|right| {
                        try_parse(right)
                    })
                })
        }
    }
}

fn main() {
    let comms = read_comms();
    // println!("{:#?}", comms);
    let nvalid = comms.messages
        .iter()
        .filter_map(|message| {
            let result = has_parse(&comms.grammar, message, 0);
            println!("{} {} {:?}", message.iter().cloned().collect::<String>(), message.len(), result);
            result
        })
        .count();
    println!("{}", nvalid);
}
