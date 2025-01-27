use core::cmp::Ordering;

use arrayvec::ArrayVec;
use defmt::{debug, error, info};

use embassy_rp::pac::common::R;
use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::multi::fold_many1;
use nom::sequence::tuple;
use nom::{branch::alt, combinator::map_res};
use nom::{IResult, InputIter, InputLength, InputTake};

use super::utils::parse::{integer, newline};

type ResultType = u64;

const FULL: &[u8] = include_bytes!("../../input/day2.full");
const SAMPLE: &[u8] = include_bytes!("../../input/day2.sample");

pub struct Solution {}
impl super::utils::Solution for Solution {
    fn new() -> impl super::utils::Solution {
        Self {}
    }

    fn run_sample(&self) {
        run("sample", SAMPLE)
    }

    fn run_full(&self) {
        run("full", FULL)
    }
}

fn run(label: &'static str, data: &[u8]) {
    info!("{} start parsing", label);
    let mut it = nom::combinator::iterator(data, parse_line);
    let mut step1_answer = 0;
    let mut step2_answer = 0;
    for (row, line) in (&mut it).enumerate() {
        info!("{} {}: {}", label, row, line.len());
        if is_safe_part1(line.as_ref()) {
            step1_answer += 1;
            step2_answer += 1;
        } else if (0..line.len()).any(|skip| is_safe_part2(line.as_ref(), skip)) {
            step2_answer += 1;
        }
    }
    match it.finish() {
        Ok((r, _)) => {
            if !r.is_empty() {
                panic!("{} residual size: {} of {}", label, r.len(), data.len())
            }
        }
        Err(_e) => error!("{} error", label),
    }

    info!("{} part1 answer: {}", label, step1_answer);
    info!("{} part2 answer: {}", label, step2_answer);
}

fn is_safe_part1(report: &[ResultType]) -> bool {
    let mut dir = Ordering::Equal;
    let mut last = 0;
    for (i, cur) in report.iter().enumerate() {
        if i == 0 {
            last = *cur;
            continue;
        }
        if last == *cur {
            return false;
        }
        if last > *cur {
            if dir == Ordering::Less {
                return false;
            }
            if last - *cur > 3 {
                return false;
            }
            dir = Ordering::Greater;
        }
        if last < *cur {
            if dir == Ordering::Greater {
                return false;
            }
            if *cur - last > 3 {
                return false;
            }
            dir = Ordering::Less;
        }
        last = *cur;
    }
    true
}
fn is_safe_part2(report: &[ResultType], skip: usize) -> bool {
    let mut dir = Ordering::Equal;
    let mut last = None;
    for (i, cur) in report.iter().enumerate() {
        if i == skip {
            continue;
        }
        if last.is_none() {
            last = Some(*cur);
            continue;
        }
        if last.unwrap() == *cur {
            return false;
        }
        if last.unwrap() > *cur {
            if dir == Ordering::Less {
                return false;
            }
            if last.unwrap() - *cur > 3 {
                return false;
            }
            dir = Ordering::Greater;
        }
        if last.unwrap() < *cur {
            if dir == Ordering::Greater {
                return false;
            }
            if *cur - last.unwrap() > 3 {
                return false;
            }
            dir = Ordering::Less;
        }
        last = Some(*cur);
    }
    true
}

fn list_number<RT, const C: usize>(input: &[u8]) -> IResult<&[u8], ArrayVec<RT, C>>
where
    RT: core::convert::TryFrom<i32>
        + core::convert::TryFrom<u8>
        + core::ops::MulAssign<RT>
        + core::ops::AddAssign
        + core::default::Default,
    <RT as core::convert::TryFrom<u8>>::Error: core::fmt::Debug,
    <RT as core::convert::TryFrom<i32>>::Error: core::fmt::Debug,
{
    fold_many1(
        tuple((integer::<RT>, opt(tag(b" ")))),
        ArrayVec::new,
        |mut acc: ArrayVec<RT, C>, item| {
            acc.push(item.0);
            acc
        },
    )(input)
}

fn parse_line(input: &[u8]) -> IResult<&[u8], ArrayVec<ResultType, 50>> {
    map_res(
        nom::sequence::tuple((list_number, newline)),
        |(answer, _)| Ok::<_, &[u8]>(answer),
    )(input)
}
