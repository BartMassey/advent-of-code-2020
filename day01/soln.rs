// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 1.  
//! Bart Massey 2020

use aoc::*;

fn find_sum(nums: &[u64], target: u64, count: usize, cur: u64) -> Option<Vec<u64>> {
    if count == 0 {
        if cur == target {
            return Some(Vec::new());
        }
        return None;
    }
    for (i, &n) in nums.iter().enumerate() {
        if let Some(mut v) = find_sum(&nums[i + 1..], target, count - 1, cur + n) {
            v.push(n);
            return Some(v);
        }
    }
    None
}

pub fn main() {
    let nums: Vec<u64> = input_lines().map(|l| l.parse().unwrap()).collect();
    let soln = match get_part() {
        Part1 => find_sum(&nums, 2020, 2, 0).unwrap(),
        Part2 => find_sum(&nums, 2020, 3, 0).unwrap(),
    };
    let p: u64 = soln.iter().cloned().product();
    println!("{}", p);
}
