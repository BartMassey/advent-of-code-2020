# Advent Of Code 2020: Tutorial Solutions in Rust
Copyright (c) 2020 Bart Massey

Herein lie Rust solutions to the problems of the 2020
[Advent of Code](http://adventofcode.com) through Day 19
Part 1.

Advent of Code is a fun exercise up to a point, and I thank
the author and others involved for their work. Thanks also
to `relsqui` for pointing me at this back in 2015.

## Previously

* [2019](http://github.com/BartMassey/advent-of-code-2019)
  in Rust (incomplete)
* [2018](http://gitlab.com/BartMassey/advent-of-code-2018)
  in Javascript (incomplete)
* [2017](http://gitlab.com/BartMassey/advent-of-code-2017)
  in Go
* [2016](http://github.com/BartMassey/advent-of-code-2016)
  in Rust
* [2015](http://github.com/BartMassey/advent-of-code-2015)
  in Haskell

## Organization

The solutions are in directories named `day01` through
`day19`. For each solution, I include cleaned-up Rust
code. There is a `README.md` in every problem directory
containing algorithm descriptions, comments and usage
instructions. I used to included the problem descriptions
and my specific `input.txt`, but apparently the authors of
AoC don't want me to do that.

The solutions load library code from the included `libaoc`
crate. See its documentation for details.

## Code Quality

There are no special system tests written for this code
other than the ones provided as part of the problem ---
there are occasional unit tests. I regard passing both parts
of a day's problem as strong validation, although I've been
wrong about this in the past. More tests should get written.

These programs are not production-quality: it is considered
acceptable to panic on erroneous input.

## Goals

The goals of these solutions are to:

* Provide correct solutions with reasonable runtimes.

* Illustrate reasonable solution strategies.

* Illustrate the use of Rust in problem-solving.

As always I expect to learn some Rust and a little bit of
software engineering I should already have known writing
these.

## Infrastructure

There's some engineering infrastructure here in the form of
the `template` directory and the `mkday.sh` and other shell
scripts.  These speed each day's setup considerably. At the
beginning of each day I `sh mkday.sh`. (The day number is
tracked automatically but can be overwritten on the command
line.)

You can get times for all parts of all days with `sh
times.sh` (will build before timing). This also verifies
that everything runs.  This is a Rust workspace: you can use
Cargo commands at top-level on the entire workspace. `cargo
clean` is especially useful here — Rust `target` directories
are huge.

## Misc

These solutions deserve a much more thorough top-level
description than I usually have the energy to
write. Apologies.

---

This work is licensed under the "MIT License".  Please see
the file `LICENSE` in the source distribution of this
software for license terms.
