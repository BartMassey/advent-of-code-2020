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

peg::parser!{
    grammar ev1() for [Token] {
        rule number() -> i64
            = n:$[Num(_)] {
                match n[0] {
                    Num(n) => n,
                    _ => unreachable!(),
                }
            }

        pub rule arithmetic() -> i64 = precedence!{
            x:(@) [Times] y:@ { x * y }
            x:(@) [Plus] y:@ { x + y }
            --
            n:number() { n }
            [LParen] e:arithmetic() [RParen] { e }
        }
    }
}

fn eval1(Eqn(eqn): &Eqn) -> i64 {
    ev1::arithmetic(eqn).unwrap()
}

peg::parser!{
    grammar ev2() for [Token] {
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
    ev2::arithmetic(eqn).unwrap()
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
