# Advent of Code 2020: Day 17
Bart Massey

---

Exercise in difficult debugging, mostly.

Reduced copy-paste in my solution by using const
generics. Fun. Requires nightly to compile. Easiest
way is probably

    rustup override set nightly

in this directory.

Added page-flipping in an attempt to improve performance
through memory locality. Makes it slightly slower for some
inscrutable reason.

---

Solution to [this problem](https://adventofcode.com/2020/day/17).

Save your problem input to `input.txt` and run with

    cargo run --release 1 <input.txt
    cargo run --release 2 <input.txt

---

This program is licensed under the "MIT License".
Please see the file LICENSE in this distribution
for license terms.
