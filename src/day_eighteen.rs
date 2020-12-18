use crate::file_util::read_non_blank_lines;
use std::str::{Chars, FromStr};
use std::iter::{once};
use itertools::Itertools;
use crate::day_eighteen::Token::{Number, LeftParen, RightParen, Operation};
use crate::day_eighteen::OperationType::{Multiply, Add};

#[derive(Eq, PartialEq, Debug, Clone)]
enum Token { LeftParen, RightParen, Operation(OperationType), Number(usize) }
#[derive(Eq, PartialEq, Debug, Clone)]
enum OperationType {
    Multiply, Add
}
#[derive(Eq, PartialEq)]
enum OperationPrecedence {
    None,
    AddWins
}

impl OperationType {
    fn apply(&self, first: usize, second: usize) -> usize {
        match self {
            Multiply => first * second,
            Add => first + second
        }
    }
}

fn to_tokens(chars: &mut Chars) -> Vec<Token> {
    let mut buff = Vec::new();
    while let Some(x) = chars.next() {
        match x {
            '(' => buff.push(LeftParen),
            ')' => buff.push(RightParen),
            '+' => buff.push(Operation(Add)),
            '*' => buff.push(Operation(Multiply)),
            _ => {
                if let Ok(v) = usize::from_str(
                    &once(x)
                        .chain(chars.by_ref().peeking_take_while(|x| x.is_ascii_digit()))
                        .join("")
                        .trim()
                ) { buff.push(Number(v)) };
            }
        }
    }
    buff
}


fn solve(tokens: &[Token], precedence: OperationPrecedence) -> Option<usize> {
    let mut stack = Vec::new();
    let mut post_fix = Vec::new();
    for token in tokens.iter() {
        match token {
            Number(_) => post_fix.push(token.clone()),
            LeftParen => {
                stack.push(token.clone());
            },
            RightParen => {
                while let Some(prev) = stack.pop() {
                    if prev == LeftParen {
                        break;
                    }
                    post_fix.push(prev);
                }
            },
            Operation(op_type) => {
                while let Some(Operation(prev_op_type)) = stack.last() {
                    if precedence == OperationPrecedence::AddWins && *op_type == Add && *prev_op_type != Add {
                        break;
                    }
                    if let Some(op) = stack.pop() {
                        post_fix.push(op);
                    }
                }
                stack.push(token.clone());
            }
        }
    }
    while let Some(token) = stack.pop() {
        post_fix.push(token);
    }
    let mut buff = Vec::new();
    for token in post_fix.iter() {
        match token {
            Operation(op_type) => {
                if let Some(Number(last)) = buff.pop() {
                    if let Some(Number(second_last)) = buff.pop() {
                        buff.push(Number(op_type.apply(second_last, last)));
                    }
                }
            },
            _ => buff.push(token.clone())
        }
    }
    if let Some(Number(x)) = buff.last() {
        Some(*x)
    } else {
        None
    }
}

#[allow(dead_code)]
pub fn run_day_eighteen() {
    let result: (usize, usize) = read_non_blank_lines("assets/day_eighteen")
        .map(|x| to_tokens(&mut x.chars()))
        .filter_map(|x|
             Some(
                 (
                    solve(&x, OperationPrecedence::None)?,
                    solve(&x, OperationPrecedence::AddWins)?
                )
             )
        )
        .fold((0, 0), |prev, (first, second)| (prev.0 + first, prev.1 + second));

    println!("Part 1 {} and Part 2 {}", result.0, result.1)
}
