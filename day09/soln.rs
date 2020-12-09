// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 9.  
//! Bart Massey 2020

use std::cmp::Ordering;

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

fn find_subseq(seq: &[u64], invalid: u64) -> (usize, usize) {
    let mut sum = seq[0];
    let mut i = 0;
    let mut j = 0;
    let nseq = seq.len();
    while i < nseq {
        match u64::cmp(&sum, &invalid) {
            Ordering::Less => {
                j += 1;
                sum += seq[j];
            }
            Ordering::Greater => {
                sum -= seq[i];
                i += 1;
            }
            Ordering::Equal => {
                return (i, j);
            }
        }
    }
    panic!("no subseq");
}

fn main() {
    let seq: Vec<u64> = input_lines().map(|l| l.parse().unwrap()).collect();
    let (part, args) = get_part_args();
    let pre_len: usize = args[0].parse().unwrap();
    let unmatched = find_unmatched(&seq, pre_len);
    match part {
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
