#![no_std]
#![allow(unused_imports)]
//extern crate alloc;
extern crate core;

use core::fmt::Debug;
//use alloc::string::{String, ToString};
//use alloc::vec::Vec;
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
use tinyvec::Array;
use tinyvec::ArrayVec;

use log::{debug, info};
use utils::Solution as _;

pub type ResultType = u64;

#[derive(Default)]
pub struct Solution {
    inputs: ArrayVec<[ArrayVec<[u8; 10240]>; 10]>,
}
impl Debug for Solution {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Solution")
            .field("inputs", &self.inputs.len())
            .finish()
    }
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
        let line = line.bytes().fold(ArrayVec::new(), |mut acc, v| {
            acc.push(v);
            acc
        });
        self.inputs.push(line);
        Ok(())
    }

    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Result<Self::ResultType, utils::Error> {
        let mut total = 0;

        for input in &self.inputs {
            let mut input = input.as_ref();
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
                };
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
            let mut input = input.as_ref();
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
                };
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

fn op_mul(input: &[u8]) -> IResult<&[u8], Op> {
    map_res(
        nom::sequence::tuple((tag("mul("), number, tag(","), number, tag(")"))),
        |(_, a, _, b, _)| Ok::<_, &[u8]>(Op::Mul(a, b)),
    )(input)
}
fn op_do(input: &[u8]) -> IResult<&[u8], Op> {
    map_res(tag("do()"), |_| Ok::<_, &[u8]>(Op::Do))(input)
}
fn op_dont(input: &[u8]) -> IResult<&[u8], Op> {
    map_res(tag("don't()"), |_| Ok::<_, &[u8]>(Op::Dont))(input)
}
fn number(input: &[u8]) -> IResult<&[u8], usize> {
    map_res(digit1, |digits: &[u8]| {
        Ok::<usize, &[u8]>(digits.iter().fold(0_usize, |mut acc, v| {
            acc *= 10;
            acc += (v - b'0') as usize;
            acc
        }))
    })(input)
}
fn onechar(input: &[u8]) -> IResult<&[u8], Op> {
    map_res(anychar, |c| Ok::<_, &[u8]>(Op::Char(c)))(input)
}
fn parse(input: &[u8]) -> IResult<&[u8], Op> {
    alt((op_mul, op_do, op_dont, onechar))(input)
}
