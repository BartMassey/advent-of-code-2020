# Advent of Code 2020: Day 1
Bart Massey

---
Good warmup. I generalized my solution to handle sums of
arbitrary length using **The Power Of Recursion.** I then
examined `input.txt` and sped things up to negligible by:

* Removing input numbers that add to more than 2020 when
  added to the smallest input (most of them). This pretty
  much takes care of Part 1.

* Searching in descending order and skipping when the
  partial sum is more than 2020. This pretty much takes care
  of Part 2.

Given these tricks, the whole thing could be done
pencil-and-paper in a couple of minutes.

Solution to
[this problem](https://adventofcode.com/2020/day/1).

Save your problem input to `input.txt` and run with

    cargo run --release 1 <input.txt
    cargo run --release 2 <input.txt

---

This program is licensed under the "MIT License".
Please see the file LICENSE in this distribution
for license terms.
