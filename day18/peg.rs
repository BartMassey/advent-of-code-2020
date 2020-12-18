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
            x:(@) [Plus] y:@ { x + y }
            --
            x:(@) [Times] y:@ { x * y }
            --
            n:number() { n }
            [LParen] e:arithmetic() [RParen] { e }
        }
    }
}

fn main() {
    let test = [Num(3), Times, Num(2)];
    println!("{}", eval::arithmetic(&test).unwrap());
}
