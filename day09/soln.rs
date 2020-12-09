// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 9.  
//! Bart Massey 2020

use std::collections::HashMap;

use aoc::*;

fn has_sum(seq: &[u64], v: u64) -> bool {
    for i in seq {
        for j in seq {
            if j == i {
                continue;
            }
            if i + j == v {
                return true;
            }
        }
    }
    false
}

fn find_unmatched(seq: &[u64], pre_len: usize) -> u64 {
    let nseq = seq.len();
    for i in pre_len..nseq {
        let subseq = &seq[i - pre_len..i];
        if !has_sum(subseq, seq[i]) {
            return seq[i];
        }
    }
    panic!("no unmatched");
}

type Memo = HashMap<(usize, usize), u64>;

fn find_sum(memo: &mut Memo, seq: &[u64], i: usize, j: usize) {
    if memo.contains_key(&(i, j)) {
        return;
    }
    let nseq = seq.len();
    assert!(i <= j && j < nseq && i < nseq);
    if i == j {
        memo.insert((i, j), seq[i]);
        return;
    }
    find_sum(memo, seq, i + 1, j);
    memo.insert((i, j), seq[i] + memo[&(i + 1, j)]);
}

fn find_subseq(seq: &[u64], invalid: u64) -> (usize, usize) {
    let mut memo: Memo = HashMap::new();
    let nseq = seq.len();
    for i in 0..nseq {
        for j in i..nseq {
            find_sum(&mut memo, seq, i, j);
            let sum = memo[&(i, j)];
            //println!("{} {} {}", i, j, sum);
            if sum > invalid {
                break;
            }
            if sum == invalid {
                return (i, j);
            }
        }
    }
    panic!("no subseq");
}

fn main() {
    let seq: Vec<u64> = input_lines().map(|l| l.parse().unwrap()).collect();
    let unmatched = find_unmatched(&seq, 25);
    match get_part() {
        Part1 => {
            println!("{}", unmatched);
        },
        Part2 => {
            let (i, j) = find_subseq(&seq, unmatched);
            let subseq = &seq[i..=j];
            let smallest = subseq.iter().cloned().min().unwrap();
            let largest = subseq.iter().cloned().max().unwrap();
            println!("{}", smallest + largest);
        },
    }
}
