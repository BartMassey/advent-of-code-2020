# Advent of Code 2020: Day 9
Bart Massey

---

Realistically my difficulties in correctly reading the
problem description were the biggest time-sink on this one,
in spite of my trying to be careful.

Looked like Part 2 was finally the dreaded dynamic
programming problem. However, I found a sliding-window
algorithm that looked to be both faster and simpler. Tried
my algorithm, didn't seem to work, convinced myself it was
wrong. Went on to spend a bunch of time on a dynamic
programming solution that I knew would work. Solved Part 2.

After I'd debugged everything, I went back and
re-implemented my original algorithm. Worked fine, much
simpler. Both implementations run too fast to accurately
measure on the given input without a lot of work.

See the branch `day09-dynamic-programming` in this repo for
the previous solution. Sigh.

I really wish the test instance wouldn't have
different parameters than the input instance: it is easy to
forget to switch between the two. Ideally, the instance
would just start with the window length. Anyhow, added an
argument to handle this, as one does.

This is *not* a decryption problem. I don't know why it is
labeled as one.

---

Solution to [this problem](https://adventofcode.com/2020/day/9).

Save your problem input to `input.txt` and run with

    cargo run --release 1 25 <input.txt
    cargo run --release 2 25 <input.txt

---

This program is licensed under the "MIT License".
Please see the file LICENSE in this distribution
for license terms.
