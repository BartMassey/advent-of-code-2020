# Advent of Code 2020: Day 16
Bart Massey

---

This is the kind of problem I hate. Parsing the input
properly took almost half an hour by itself. Solving Part 2
took me 2 hours.

I foolishly assumed I needed a state-space search, but chose
poorly about the state space. More specifically, turns out I
had a bug where I was ordering the values backward. This
kind of performance bug is *so hard* to spot on the fly.

Turns out the input was rigged in a fashion not documented
anywhere in the problem statement.  This kind of baloney
"examine-the-input" analysis that comes out of the blue is
just no fun. If I hadn't bugged my search I wouldn't have
noticed. But I did, so I did.

May be done with AoC for this year. We'll see tomorrow.

---

Solution to [this problem](https://adventofcode.com/2020/day/16).

Save your problem input to `input.txt` and run with

    cargo run --release 1 <input.txt
    cargo run --release 2 <input.txt

---

This program is licensed under the "MIT License".
Please see the file LICENSE in this distribution
for license terms.
