// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 16.  
//! Bart Massey 2020

use std::collections::{HashSet, HashMap};
use std::str::FromStr;

use aoc::*;

// Range is inclusive.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Range {
    lower: u64,
    upper: u64,
}

impl Range {
    fn matches(&self, v: u64) -> bool {
        v >= self.lower && v <= self.upper
    }
}

impl FromStr for Range {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bs = s.split('-').map(|b| b.parse());
        let lower = bs.next().unwrap()?;
        let upper = bs.next().unwrap()?;
        Ok(Range { lower, upper })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Ranges(HashSet<Range>);

impl Ranges {
    fn matches(&self, v: u64) -> bool {
        self.0.iter().any(|r| r.matches(v))
    }
}

impl FromStr for Ranges {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ranges = s
            .split(" or ")
            .map(|r| r.parse())
            .collect::<Result<HashSet<Range>, _>>()?;
        Ok(Ranges(ranges))
    }
}

type Ticket = Vec<u64>;

#[derive(Debug, Clone)]
struct Record {
    fields: HashMap<String, Ranges>,
    my: Ticket,
    others: Vec<Ticket>,
}

fn read_record() -> Record {
    use std::io::Read;
    let mut stdin = std::io::stdin();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();

    let sections: Vec<&str> = input.split("\n\n").collect();
    assert_eq!(sections.len(), 3);
    let fields = sections[0]
        .split('\n')
        .map(|l| {
            let terms: Vec<&str> = l.split(": ").collect();
            assert_eq!(terms.len(), 2);
            let name = terms[0].to_string();
            let ranges: Ranges = terms[1].parse().unwrap();
            (name, ranges)
        })
        .collect();

    let read_ticket = |s: &str| s.split(',')
        .map(|f| f.parse().unwrap()).collect::<Ticket>();

    let mut ticker = sections[1].split('\n');
    assert_eq!(ticker.next().unwrap(), "your ticket:");
    let my: Ticket = read_ticket(ticker.next().unwrap());

    let mut ticker = sections[2].split('\n');
    assert_eq!(ticker.next().unwrap(), "nearby tickets:");
    let others = ticker.map(read_ticket).collect();

    Record { fields, my, others }
}

fn main() {
    let record = read_record();
    // println!("{:#?}", record);
    match get_part() {
        Part1 => {
            let sum: u64 = record.others
                .iter()
                .flat_map(|t| {
                    t.iter()
                        .filter(|&&v| {
                            !record.fields.values().any(|rs| {
                                rs.matches(v)
                            })
                        })
                        .cloned()
                })
                .sum();
            println!("{}", sum);
        }
        Part2 => {
            let mut good_tickets: Vec<&Ticket> = record.others
                .iter()
                .filter(|&t| {
                    t.iter()
                        .all(|&v| {
                            record.fields.values().any(|rs| {
                                rs.matches(v)
                            })
                        })
                })
                .collect();
            good_tickets.push(&record.my);

            let nfields = record.fields.len();
            let mut fields: HashMap<&str, HashSet<usize>> = record.fields
                .iter()
                .map(|(name, ranges)| {
                    let h: HashSet<usize> = (0..nfields)
                        .filter(|&p| {
                            good_tickets
                                .iter()
                                .all(|t| ranges.matches(t[p]))
                        })
                        .collect();
                    (name.as_str(), h)
                })
                .collect();

            let mut posns: HashMap<&str, usize> = HashMap::new();
            while !fields.is_empty() {
                // Most constrained field.
                let (name, cands) = fields
                    .iter()
                    .min_by_key(|&(_, v)| v.len())
                    .unwrap();
                let name = name.clone();
                let cands = cands.clone();
                assert_eq!(cands.len(), 1);
                let pos = cands.into_iter().next().unwrap();
                posns.insert(name, pos);
                fields.remove(name);
                for (_, ref mut cands) in &mut fields {
                    cands.remove(&pos);
                }
            }

            let p: u64 = posns
                .iter()
                .filter_map(|(&n, &p)| {
                    if n.starts_with("departure") {
                        Some(record.my[p])
                    } else {
                        None
                    }
                })
                .product();
            println!("{}", p);
        }
    }
}
