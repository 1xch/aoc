use logos::{Lexer, Logos};
use std::{collections::HashMap, iter::Peekable};

fn main() {
    //let mut file = File::open("./input.txt").unwrap();

    //let mut data = String::new();

    //file.read_to_string(&mut data).unwrap();

    part1(include_str!("input.txt"));
    part2(include_str!("input.txt"));
}

#[derive(Logos, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Token {
    #[token("+")]
    Plus,
    #[token("*")]
    Star,

    #[regex("[0-9]+", |lex| lex.slice().parse())]
    Num(u64),

    #[token("(")]
    LeftParen,

    #[token(")")]
    RightParen,

    #[error]
    #[regex("[ \n]+", logos::skip)]
    Error,
}

impl Token {
    fn is_op(&self) -> bool {
        matches!(self, Token::Plus | Token::Star)
    }
}

fn expr(lex: &mut Peekable<Lexer<Token>>, map: &HashMap<Token, (u8, u8)>) -> u64 {
    fn expr_bp(
        lex: &mut Peekable<Lexer<Token>>,
        map: &HashMap<Token, (u8, u8)>,
        min_bp: u8,
    ) -> u64 {
        let mut lhs = match lex.next().unwrap() {
            Token::Num(val) => val,
            Token::LeftParen => {
                let result = expr_bp(lex, map, 0);
                lex.next();

                result
            }
            _ => unreachable!(),
        };

        loop {
            let op = match lex.peek().cloned() {
                None => break,
                Some(token) if token.is_op() => token,
                Some(Token::RightParen) => break,
                _ => unreachable!(),
            };

            let (l_bp, r_bp) = map[&op];
            if l_bp < min_bp {
                break;
            }

            lex.next();
            let rhs = expr_bp(lex, map, r_bp);

            match op {
                Token::Plus => {
                    lhs += rhs;
                }
                Token::Star => {
                    lhs *= rhs;
                }
                _ => unreachable!(),
            }
        }

        lhs
    }

    expr_bp(lex, map, 0)
}

fn part1(input: &str) {
    println!("Part 1");

    let mut map = HashMap::new();
    map.insert(Token::Star, (1, 2));
    map.insert(Token::Plus, (1, 2));

    let mut result = 0;
    for line in input.lines() {
        let mut lex = Token::lexer(&line).peekable();

        result += expr(&mut lex, &map);
    }

    println!("{}", result);
}

fn part2(input: &str) {
    println!("Part 2");

    let mut map = HashMap::new();
    map.insert(Token::Star, (1, 2));
    map.insert(Token::Plus, (3, 4));

    let mut result = 0;
    for line in input.lines() {
        let mut lex = Token::lexer(&line).peekable();

        result += expr(&mut lex, &map);
    }

    println!("{}", result);
}
