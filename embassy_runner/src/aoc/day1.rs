use defmt::{debug, info};

use nom::bytes::complete::tag;
use nom::bytes::complete::take_while;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::multi::fold_many1;
use nom::IResult;

use scapegoat::SgMap;

use crate::aoc::utils::FixedVec;

pub type ResultType = u64;

const FULL: &'static [u8] = include_bytes!("../../../input/day1.full");
const SAMPLE: &'static [u8] = include_bytes!("../../../input/day1.sample");

pub fn run_sample() {
    run("sample", SAMPLE)
}

pub fn run_full() {
    run("full", FULL)
}

fn run(label: &'static str, data: &[u8]) {
    info!("{} start parsing", label);
    let mut left: FixedVec<_, 1024> = FixedVec::new();
    let mut right: FixedVec<_, 1024> = FixedVec::new();

    let mut it = nom::combinator::iterator(data, parse_line);
    for (a, b) in &mut it {
        debug!("{} a:{} b:{}", label, a, b);
        left.push(a);
        right.push(b);
    }
    if let Err(_) = it.finish() {
        info!("{} error", label);
    } else {
        info!("{} processing", label);
        left.as_mut_ref().sort_unstable();
        right.as_mut_ref().sort_unstable();

        let answer: ResultType = left
            .iter()
            .zip(right.iter())
            .map(|(a, b)| distance(a, b))
            .sum();
        info!("{} part1 answer: {}", label, answer);

        let right_count = right.iter().fold(
            SgMap::new(),
            |mut acc: SgMap<ResultType, ResultType, 1024>, value| {
                let entry: &mut ResultType = acc.entry(*value).or_default();
                *entry += 1;
                acc
            },
        );

        let answer: ResultType = left
            .iter()
            .map(|v| v * right_count.get(v).cloned().unwrap_or_default())
            .sum();

        info!("{} part2 answer: {}", label, answer);
    }
}

fn distance(a: &ResultType, b: &ResultType) -> ResultType {
    if a < b {
        b - a
    } else {
        a - b
    }
}

fn integer(input: &[u8]) -> IResult<&[u8], ResultType> {
    map_res(digit1, |digits: &[u8]| {
        Ok::<ResultType, &[u8]>(digits.iter().fold(0 as ResultType, |mut acc, v| {
            acc *= 10;
            acc += (v - b'0') as ResultType;
            acc
        }))
    })(input)
}

fn newline(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while(|c| c == b'\n' || c == b'\r')(input)
}

fn whitespace(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while(|c| c == b' ' || c == b'\t')(input)
}

fn parse_line(input: &[u8]) -> IResult<&[u8], (ResultType, ResultType)> {
    map_res(
        nom::sequence::tuple((integer, whitespace, integer, newline)),
        |(lhs, _, rhs, _)| Ok::<_, &[u8]>((lhs, rhs)),
    )(input)
}
