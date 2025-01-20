use defmt::{debug, info};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take;
use nom::character::complete::digit1;
use nom::combinator::iterator;
use nom::combinator::map_res;
use nom::IResult;

pub type ResultType = u64;

const FULL: &'static [u8] = include_bytes!("../../../input/day3.full");
const SAMPLE: &'static [u8] = include_bytes!("../../../input/day3.sample");

pub fn run_sample() {
    run("sample", SAMPLE)
}

pub fn run_full() {
    run("full", FULL)
}

fn run(label: &'static str, data: &[u8]) {
    info!("{} start parsing", label);
    let mut it = iterator(data, parse);
    let mut step1_answer = 0;
    let mut step2_answer = 0;
    let mut enabled = true;
    for v in &mut it {
        match v {
            Op::Noop => {}
            Op::Mul(mul) => {
                step1_answer += mul;
                if enabled {
                    step2_answer += mul;
                }
            }
            Op::Disable => {
                enabled = false;
            }
            Op::Enable => {
                enabled = true;
            }
        }
    }
    info!("{} step1 answer = {}", label, step1_answer);
    info!("{} step2 answer = {}", label, step2_answer);
}

enum Op {
    Noop,
    Mul(ResultType),
    Enable,
    Disable,
}
fn number(input: &[u8]) -> IResult<&[u8], ResultType> {
    map_res(digit1, |digits: &[u8]| {
        Ok::<ResultType, &[u8]>(digits.iter().fold(0 as ResultType, |mut acc, v| {
            acc *= 10;
            acc += (v - b'0') as ResultType;
            acc
        }))
    })(input)
}

fn mul(input: &[u8]) -> IResult<&[u8], Op> {
    map_res(
        nom::sequence::tuple((tag(b"mul("), number, tag(b","), number, tag(b")"))),
        |(_, lhs, _, rhs, _)| Ok::<_, &[u8]>(Op::Mul(lhs * rhs)),
    )(input)
}

fn on(input: &[u8]) -> IResult<&[u8], Op> {
    map_res(tag(b"do()"), |_| Ok::<_, &[u8]>(Op::Enable))(input)
}
fn off(input: &[u8]) -> IResult<&[u8], Op> {
    map_res(tag(b"don't()"), |_| Ok::<_, &[u8]>(Op::Disable))(input)
}
fn skip(input: &[u8]) -> IResult<&[u8], Op> {
    map_res(take(1_usize), |_| Ok::<_, &[u8]>(Op::Noop))(input)
}

fn parse(input: &[u8]) -> IResult<&[u8], Op> {
    alt((mul, on, off, skip))(input)
}
