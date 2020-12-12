// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 12.  
//! Bart Massey 2020

use aoc::*;

use aoc::Dirn::*;
use aoc::Rot::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    D(Dirn),
    R(Rot),
    F,
}

fn read_directions() -> Vec<(Move, i64)> {
    let re = Reparse::new("^([NSEWLRF])([1-9][0-9]*)$");
    input_lines()
        .map(|l| {
            let p = re.parse(&l).unwrap();
            let m = match p.get(1) {
                'N' => Move::D(Up),
                'S' => Move::D(Down),
                'E' => Move::D(Right),
                'W' => Move::D(Left),
                'L' => Move::R(CCW),
                'R' => Move::R(CW),
                'F' => Move::F,
                _ => panic!("unexpected move"),
            };
            let d = p.get(2);
            (m, d)
        })
        .collect()
}

fn main() {
    let input = read_directions();
    // println!("{:#?}", input);
    match get_part() {
        Part1 => {
            let mut facing = Right;
            let mut posn = (0, 0);
            for (m, d) in input {
                match m {
                    Move::D(dirn) => {
                        posn = dirn.displace(posn, d);
                    }
                    Move::R(dirn) => {
                        assert!(d % 90 == 0);
                        facing = facing.turn(dirn, d / 90);
                    }
                    Move::F => {
                        posn = facing.displace(posn, d);
                    }
                }
            }
            let travel: u64 = manhattan_distance((0, 0), posn);
            println!("{}", travel);
        }
        Part2 => {
            let mut ps = (0, 0);
            let mut pw = (-1, 10);
            //println!("s={:?} w={:?}", ps, pw);
            for (m, mut d) in input {
                match m {
                    Move::D(dirn) => {
                        pw = dirn.displace(pw, d);
                    }
                    Move::R(CCW) => {
                        while d > 0 {
                            // (1, 0) -> (0, -1) -> (-1, 0) -> (0, 1)
                            //
                            // | 0 1| rw 1  0 -1 0
                            // |-1 0| cw 0 -1  0 1
                            pw = (-pw.1, pw.0);
                            d -= 90;
                        }
                    }
                    Move::R(CW) => {
                        while d > 0 {
                            pw = (pw.1, -pw.0);
                            d -= 90;
                        }
                    }
                    Move::F => {
                        ps.0 += pw.0 * d;
                        ps.1 += pw.1 * d;
                    }
                }
                //println!("s={:?} w={:?}", ps, pw);
            }
            let travel: u64 = manhattan_distance((0, 0), ps);
            println!("{}", travel);
        }
    }
}
