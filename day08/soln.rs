// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 8.  
//! Bart Massey 2020

use std::collections::{HashMap, HashSet};

use aoc::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Opcode {
    Nop,
    Acc,
    Jmp,
}
use Opcode::*;

#[derive(Debug)]
struct Insn {
    op: Opcode,
    opnd: isize,
}

fn read_program() -> Vec<Insn> {
    let parser = Reparse::new("^([a-z][a-z][a-z]) ([-+][0-9]+)$");

    let names = vec![
        ("nop", Nop),
        ("acc", Acc),
        ("jmp", Jmp),
    ];
    let names: HashMap<&'static str, Opcode> =
        names.into_iter().collect();

    let mut program = Vec::new();
    for line in input_lines() {
        let captures = parser.parse(&line).unwrap();
        let opcode: String = captures.get(1);
        let opnd: isize = captures.get(2);
        let op = names[&*opcode];
        let insn = Insn { op, opnd };
        program.push(insn);
    }
    program
}

fn run_program(program: &[Insn]) -> (isize, isize) {
    let nprogram = program.len();
    let mut visited = HashSet::new();
    let mut pc: isize = 0;
    let mut acc = 0;
    while pc >= 0 && pc < nprogram as isize && !visited.contains(&pc) {
        visited.insert(pc);
        match &program[pc as usize] {
            Insn { op: Nop, .. } => {
                pc += 1;
            }
            Insn { op: Acc, opnd } => {
                acc += opnd;
                pc += 1;
            }
            Insn { op: Jmp, opnd } => {
                pc += opnd;
            }
        }
    }
    (pc, acc)
}

fn main() {
    let mut program = read_program();
    match get_part() {
        Part1 => {
            let (_, acc) = run_program(&program);
            println!("{}", acc);
        },
        Part2 => {
            let nprogram = program.len();

            let mut try_sub = |old, new, i: usize| {
                if program[i].op == old {
                    program[i].op = new;
                    let (pc, acc) = run_program(&program);
                    if pc == nprogram as isize {
                        return Some(acc);
                    }
                    program[i].op = old;
                }
                None
            };
            
            for i in 0..nprogram {
                if let Some(acc) = try_sub(Jmp, Nop, i)
                    .or_else(|| try_sub(Nop, Jmp, i))
                {
                    println!("{}", acc);
                    return;
                }
            }
            panic!("no substitution found");
        },
    }
}
