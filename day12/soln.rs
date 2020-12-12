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
            let mut x = 0;
            let mut y = 0;
            for (m, mut d) in input {
                match m {
                    Move::D(dirn) => {
                        let (dx, dy) = dirn.disp();
                        x += dx * d;
                        y += dy * d;
                    },
                    Move::R(dirn) => {
                        while d > 0 {
                            facing = facing.turn(dirn);
                            d -= 90;
                        }
                    }
                    Move::F => {
                        let (dx, dy) = facing.disp();
                        x += dx * d;
                        y += dy * d;
                    }
                }
            }
            let travel = manhattan_distance((0, 0), (x, y));
            println!("{}", travel);
        }
        Part2 => {
            let mut xs = 0;
            let mut ys = 0;
            let mut xw = 10;
            let mut yw = -1;
            for (m, mut d) in input {
                //println!("s=({},{}) w=({},{})", xs, ys, xw, yw);
                match m {
                    Move::D(dirn) => {
                        let (dx, dy) = dirn.disp();
                        xw += dx * d;
                        yw += dy * d;
                    },
                    Move::R(CCW) => {
                        while d > 0 {
                            // (1, 0) -> (0, -1) -> (-1, 0) -> (0, 1)
                            //
                            // | 0 1| xw 1  0 -1 0
                            // |-1 0| yw 0 -1  0 1
                            let nrw = yw;
                            let ncw = -xw;
                            xw = nrw;
                            yw = ncw;
                            d -= 90;
                        }
                    }
                    Move::R(CW) => {
                        while d > 0 {
                            let nrw = -yw;
                            let ncw = xw;
                            xw = nrw;
                            yw = ncw;
                            d -= 90;
                        }
                    }
                    Move::F => {
                        xs += xw * d;
                        ys += yw * d;
                    }
                }
            }
            let travel = manhattan_distance((0, 0), (xs, ys));
            println!("{}", travel);
        }
    }
}
