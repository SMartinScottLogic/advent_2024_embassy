use core::cmp::Ordering;

use defmt::{debug, info};

use nom::bytes::complete::tag;
use nom::bytes::complete::take_while;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::multi::fold_many1;
use nom::IResult;

use crate::aoc::utils::FixedVec;

pub type ResultType = u64;

const FULL: &'static [u8] = include_bytes!("../../../input/day2.full");
const SAMPLE: &'static [u8] = include_bytes!("../../../input/day2.sample");

pub fn run_sample() {
    run("sample", SAMPLE)
}

pub fn run_full() {
    run("full", FULL)
}

fn run(label: &'static str, data: &[u8]) {
    info!("{} start parsing", label);
    let mut it = nom::combinator::iterator(data, parse_line);
    let mut row = 0;
    let mut step1_answer = 0;
    let mut step2_answer = 0;
    for line in &mut it {
        debug!("{} {}: {}", label, row, line.len());
        if is_safe_part1(line.as_ref()) {
            step1_answer += 1;
            step2_answer += 1;
        } else {
            if (0..line.len()).any(|skip| is_safe_part2(line.as_ref(), skip)) {
                step2_answer += 1;
            }
        }
        row += 1;
    }
    if let Err(_) = it.finish() {
        info!("{} error", label);
    } else {
        info!("{} part1 answer: {}", label, step1_answer);
        info!("{} part2 answer: {}", label, step2_answer);
    }
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

fn number(input: &[u8]) -> IResult<&[u8], ResultType> {
    map_res(digit1, |digits: &[u8]| {
        Ok::<ResultType, &[u8]>(digits.iter().fold(0 as ResultType, |mut acc, v| {
            acc *= 10;
            acc += (v - b'0') as ResultType;
            acc
        }))
    })(input)
}
fn list_number(input: &[u8]) -> IResult<&[u8], FixedVec<ResultType, 50>> {
    fold_many1(
        nom::sequence::tuple((whitespace, number)),
        FixedVec::new,
        |mut acc: FixedVec<ResultType, 50>, item| {
            acc.push(item.1);
            acc
        },
    )(input)
}
fn newline(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while(|c| c == b'\n' || c == b'\r')(input)
}

fn whitespace(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while(|c| c == b' ' || c == b'\t')(input)
}

fn parse_line(input: &[u8]) -> IResult<&[u8], FixedVec<ResultType, 50>> {
    map_res(
        nom::sequence::tuple((list_number, newline)),
        |(answer, _)| Ok::<_, &[u8]>(answer),
    )(input)
}
