// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 18.  
//! Bart Massey 2020

use aoc::*;

use peg;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    Plus,
    Times,
    LParen,
    RParen,
    Num(i64),
}
use Token::*;

#[derive(Debug)]
struct Eqn(Vec<Token>);

fn read_eqns() -> Vec<Eqn> {
    input_lines()
        .map(|l| {
            let chs: Vec<char> = l.chars().collect();
            let mut toks = Vec::new();
            for (i, &c) in chs.iter().enumerate() {
                match c {
                    ' ' => (),
                    '*' => toks.push(Times),
                    '+' => toks.push(Plus),
                    '(' => toks.push(LParen),
                    ')' => toks.push(RParen),
                    d if d.is_digit(10) => {
                        let mut j = i + 1;
                        while j < chs.len() && chs[j].is_digit(10) {
                            j += 1;
                        }
                        let num: String = chs[i..j].iter().cloned().collect();
                        let num: i64 = num.parse().unwrap();
                        toks.push(Num(num));
                    }
                    c => panic!("lexer found {}", c),
                }
            }
            Eqn(toks)
        })
        .collect()
}

fn eval1(Eqn(eqn): &Eqn) -> i64 {
    let mut accum: Vec<i64> = Vec::new();
    let mut opstack: Vec<Option<Token>> = Vec::new();
    let mut op: Option<Token> = None;

    fn tos(a: &mut[i64]) -> &mut i64 {
        a.last_mut().unwrap()
    }

    accum.push(0);
    for &tok in eqn {
        match tok {
            Num(n) => {
                match op {
                    None => *tos(&mut accum) = n,
                    Some(tok) => {
                        match tok {
                            Plus => *tos(&mut accum) += n,
                            Times => *tos(&mut accum) *= n,
                            _  => panic!("bad operator"),
                        }
                        op = None;
                    }
                }
            }
            Plus | Times => op = Some(tok),
            LParen => {
                accum.push(0);
                opstack.push(op);
                op = None;
            }
            RParen => {
                let n = accum.pop().unwrap();
                let xop = opstack.pop().unwrap();
                match xop {
                    Some(Plus) => *tos(&mut accum) += n,
                    Some(Times) => *tos(&mut accum) *= n,
                    None => *tos(&mut accum) = n,
                    _ => panic!("unexpected op in )"),
                }
                op = None;
            }
        }
    }
    assert!(accum.len() == 1 && op.is_none());
    accum[0]
}

peg::parser!{
    grammar eval() for [Token] {
        rule number() -> i64
            = n:$[Num(_)] {
                match n[0] {
                    Num(n) => n,
                    _ => unreachable!(),
                }
            }

        pub rule arithmetic() -> i64 = precedence!{
            x:(@) [Times] y:@ { x * y }
            --
            x:(@) [Plus] y:@ { x + y }
            --
            n:number() { n }
            [LParen] e:arithmetic() [RParen] { e }
        }
    }
}

fn eval2(Eqn(eqn): &Eqn) -> i64 {
    eval::arithmetic(eqn).unwrap()
}

fn main() {
    let eqns = read_eqns();
    // println!("{:#?}", eqns);
    match get_part() {
        Part1 => {
            let s: i64 = eqns
                .iter()
                .map(eval1)
                // .inspect(|v| println!("{}", v))
                .sum();
            println!("{}", s);
        }
        Part2 => {
            let s: i64 = eqns
                .iter()
                .map(eval2)
                // .inspect(|v| println!("{}", v))
                .sum();
            println!("{}", s);
        }
    }
}
