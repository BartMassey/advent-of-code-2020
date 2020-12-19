# Advent of Code 2020: Day 19
Bart Massey

---

Welp, spent a long time building a lexer and recursive
descent parser for this grammar. Solved Part 1 on the first
try in about an hour. Code should "just work" for Part 2,
but it doesn't: I don't have the slightest idea why after
poking at some stuff.

Having the start rule of the first example be the only thing
in the Part 1 example and input that has a nonterminal
production not having two nonterminals, but three, was a
real kick in the teeth, since I carefully examined the
input, but didn't look at the test case until too late.  A
further kick in the teeth was having to hand-edit the input
and test for Part 2: I'm still not sure I did it right after
checking a bunch of times. This is not good problem design
and presentation.

Maybe finally done with AoC 2020. We'll see.

---

Solution to [this problem](https://adventofcode.com/2020/day/19).

Save your problem input to `input.txt` and run with

    cargo run --release 1 <input.txt
    cargo run --release 2 <input.txt

---

This program is licensed under the "MIT License".
Please see the file LICENSE in this distribution
for license terms.
