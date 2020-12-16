// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 13.  
//! Bart Massey 2020

use aoc::*;
use aoc_numberfns::*;

fn read_schedule() -> (u64, Vec<Option<u64>>) {
    let mut lines = input_lines();
    let arrival = lines.next().unwrap().parse().unwrap();
    let ids = lines
        .next()
        .unwrap()
        .split(',')
        .map(|bid| {
            if bid == "x" {
                None
            } else {
                Some(bid.parse().unwrap())
            }
        })
        .collect();
    (arrival, ids)
}

fn delay(arrival: u64, id: u64) -> u64 {
    let mut t = 0;
    while t < arrival {
        t += id;
    }
    t - arrival
}

fn contest(schedule: Vec<Option<u64>>) -> u64 {
    let schedule: Vec<(u64, u64)> = schedule
        .into_iter()
        .enumerate()
        .filter_map(|(d, i)| i.map(|v| (d as u64, v)))
        .collect();

    let mut t = 0;
    let mut cycle = 1;

    for &(delay, bid) in &schedule {
        let delay = delay % bid;
        let target = (bid - delay) % bid;
        while t % bid != target {
            t += cycle;
        }
        cycle = lcm(cycle, bid);
    }

    for &(delay, bid) in &schedule {
        let delay = delay % bid;
        let target = (bid - delay) % bid;
        assert!(t % bid == target);
    }

    t
}

fn main() {
    let (arrival, ids) = read_schedule();
    // println!("{}", arrival);
    // println!("{:#?}", ids);
    match get_part() {
        Part1 => {
            let soln = ids
                .into_iter()
                .filter_map(|mid| mid)
                .map(move |id| (id, delay(arrival, id)))
                .min_by_key(|&(_, d)| d)
                .unwrap();
            println!("{}", soln.0 * soln.1);
        }
        Part2 => {
            let min_t = contest(ids);
            println!("{}", min_t);
        }
    }
}
