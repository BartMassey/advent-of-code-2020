// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 14.  
//! Bart Massey 2020

use std::collections::HashMap;

use aoc::*;
use aoc_reparse::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Insn {
    Mask { nand: u64, or: u64 },
    Store { addr: usize, value: u64 },
}
use Insn::*;

fn read_prog() -> Vec<Insn> {
    let mask_re = Reparse::new(r"^mask = ([10X]+)$");
    let store_re = Reparse::new(r"^mem\[([0-9]+)\] = ([0-9]+)$");
    input_lines()
        .map(|l| {
            if let Some(mask) = mask_re.parse(&l) {
                let mstr: String = mask.get(1);
                assert!(mstr.len() == 36);
                let mut nand = 0;
                let mut or = 0;
                for (i, c) in mstr.chars().enumerate() {
                    let i = 35 - i;
                    match c {
                        '0' => nand |= 1 << i,
                        '1' => or |= 1 << i,
                        'X' => (),
                        _ => panic!("unrecognized mask bit"),
                    }
                }
                return Mask { nand, or };
            }
            if let Some(store) = store_re.parse(&l) {
                let addr = store.get(1);
                let value = store.get(2);
                return Store { addr, value };
            }
            panic!("unmatched insn");
        })
        .collect()
}

fn run_v1(prog: &[Insn]) -> u64 {
    let mut mem: HashMap<usize, u64> = HashMap::new();
    let mut mask_nand = 0;
    let mut mask_or = 0;

    for &insn in prog {
        match insn {
            Mask { nand, or } => {
                mask_nand = nand;
                mask_or = or;
            }
            Store { addr, value } => {
                mem.insert(addr, value & !mask_nand | mask_or);
            }
        }
    }

    mem.values().sum()
}

// Many thanks to
// https://fasterthanli.me/articles/recursive-iterators-rust
// for pointing out how to make this work as an iterator.
fn gen_addrs(
    addr: usize,
    xs: usize,
    bit: usize,
) -> Box<dyn Iterator<Item = usize>> {
    for i in bit..=35 {
        if (xs >> i) & 1 == 1 {
            let zeros = gen_addrs(addr & !(1 << i), xs, i + 1);
            let zeros_ones =
                zeros.chain(gen_addrs(addr | (1 << i), xs, i + 1));
            return Box::new(zeros_ones);
        }
    }
    Box::new(std::iter::once(addr))
}

fn run_v2(prog: &[Insn]) -> u64 {
    let mut mem: HashMap<usize, u64> = HashMap::new();
    let mut mask_nand = 0;
    let mut mask_or = 0;

    for &insn in prog {
        match insn {
            Mask { nand, or } => {
                mask_nand = nand as usize;
                mask_or = or as usize;
            }
            Store { addr, value } => {
                let xs = !(mask_nand | mask_or);
                for addr in gen_addrs(addr, xs, 0) {
                    mem.insert(addr | mask_or, value);
                }
            }
        }
    }

    mem.values().sum()
}

fn main() {
    let prog = read_prog();
    // println!("{:#?}", prog);
    match get_part() {
        Part1 => {
            let msum = run_v1(&prog);
            println!("{}", msum);
        }
        Part2 => {
            let msum = run_v2(&prog);
            println!("{}", msum);
        }
    }
}
