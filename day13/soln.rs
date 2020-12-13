// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 13.  
//! Bart Massey 2020

use aoc::*;

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

    let (delay0, bid0) = schedule[0];
    let mut t = delay0;
    let mut cycle = bid0;

    for &(delay, bid) in &schedule[1..] {
        let delay = delay % bid;
        assert!(delay > 0);
        while t % bid != bid - delay {
            t += cycle;
        }
        cycle = lcm(cycle, bid);
    }

    let ok0 =
        (delay0 == 0 && t % bid0 == 0) || t % bid0 == bid0 - delay0;
    assert!(ok0);
    for &(delay, bid) in &schedule[1..] {
        let delay = delay % bid;
        assert!(t % bid == bid - delay);
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
