// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 1.  
//! Bart Massey 2020

use aoc::*;

// Return the terms of the sum, if found.
fn find_sum(
    // Numbers to try.
    nums: &[u64],
    // Target sum.
    target: u64,
    // Number of terms remaining to find.
    count: usize,
    // Current partial sum.
    cur: u64,
) -> Option<Vec<u64>> {
    // Prune if sum has gotten too large. Sort `nums` in
    // descending order for best effect.
    if cur > target {
        return None;
    }

    // Base case: no more numbers to sum.
    if count == 0 {
        if cur == target {
            return Some(Vec::new());
        }
        return None;
    }

    // Recursive case: try to extend the sum by one more term.
    for (i, &n) in nums.iter().enumerate() {
        // Remove symmetry by searching only ahead in `nums`.
        let s = find_sum(&nums[i + 1..], target, count - 1, cur + n);
        if let Some(mut v) = s {
            v.push(n);
            return Some(v);
        }
    }

    // No solution on this branch.
    None
}

fn main() {
    // Collect input.
    let mut nums: Vec<u64> =
        input_lines().map(|l| l.parse().unwrap()).collect();

    // Remove "too large" inputs.
    nums.sort_by_key(|&n| std::cmp::Reverse(n));
    let smallest = *nums.last().unwrap();
    nums.retain(move |&n| n == smallest || n + smallest <= 2020);

    // Find the sum.
    let nterms = match get_part() {
        Part1 => 2,
        Part2 => 3,
    };
    let soln = find_sum(&nums, 2020, nterms, 0).unwrap();

    // Show the product.
    let p: u64 = soln.into_iter().product();
    println!("{}", p);
}
