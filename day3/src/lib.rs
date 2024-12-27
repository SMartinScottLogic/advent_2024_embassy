#![no_std]
#![allow(unused_imports)]
extern crate alloc;
extern crate core;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::num::ParseIntError;
use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::anychar;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::terminated;

use log::{debug, info};
use utils::Solution as _;

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    inputs: Vec<String>,
}

impl TryFrom<&str> for Solution {
    type Error = ParseIntError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (id, line) in value.lines().enumerate() {
            solution.update_from_line(id, line)?;
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type ResultType = ResultType;
    type ParseError = ParseIntError;

    #[allow(unused_variables)]
    fn update_from_line(&mut self, _id: usize, line: &str) -> Result<(), Self::ParseError> {
        self.inputs.push(line.to_string());
        Ok(())
    }

    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Result<Self::ResultType, utils::Error> {
        let mut total = 0;

        for input in &self.inputs {
            let mut input = input.clone();
            loop {
                input = match parse(&input).unwrap() {
                    (s, Op::Char(c)) => {
                        debug!("skip char({})", c);
                        s
                    }
                    (s, Op::Mul(a, b)) => {
                        total += a as ResultType * b as ResultType;
                        s
                    }
                    (s, _) => s,
                }
                .to_string();
                if input.is_empty() {
                    break;
                }
            }
        }
        Ok(total)
    }

    fn answer_part2(&self, _is_full: bool) -> Result<Self::ResultType, utils::Error> {
        let mut total = 0;
        let mut enable = true;
        for input in &self.inputs {
            let mut input = input.clone();
            loop {
                input = match parse(&input).unwrap() {
                    (s, Op::Char(c)) => {
                        debug!("skip char({})", c);
                        s
                    }
                    (s, Op::Mul(a, b)) => {
                        if enable {
                            total += a as ResultType * b as ResultType;
                        }
                        s
                    }
                    (s, Op::Do) => {
                        enable = true;
                        s
                    }
                    (s, Op::Dont) => {
                        enable = false;
                        s
                    }
                }
                .to_string();
                if input.is_empty() {
                    break;
                }
            }
        }
        Ok(total)
    }
}

enum Op {
    Mul(usize, usize),
    Char(char),
    Do,
    Dont,
}

fn op_mul(input: &str) -> IResult<&str, Op> {
    map_res(
        nom::sequence::tuple((tag("mul("), number, tag(","), number, tag(")"))),
        |(_, a, _, b, _)| Ok::<_, &str>(Op::Mul(a, b)),
    )(input)
}
fn op_do(input: &str) -> IResult<&str, Op> {
    map_res(tag("do()"), |_| Ok::<_, &str>(Op::Do))(input)
}
fn op_dont(input: &str) -> IResult<&str, Op> {
    map_res(tag("don't()"), |_| Ok::<_, &str>(Op::Dont))(input)
}
fn number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |digits: &str| digits.parse::<usize>())(input)
}
fn onechar(input: &str) -> IResult<&str, Op> {
    map_res(anychar, |c| Ok::<_, &str>(Op::Char(c)))(input)
}
fn parse(input: &str) -> IResult<&str, Op> {
    alt((op_mul, op_do, op_dont, onechar))(input)
}
