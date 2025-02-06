use crate::{debug, info};
use arrayvec::ArrayVec;

use nom::{combinator::map_res, IResult};
use scapegoat::SgMap;

use super::utils::parse::{integer, newline, whitespace};

type ResultType = u64;

const FULL: &[u8] = include_bytes!("../../input/day1.full");
const SAMPLE: &[u8] = include_bytes!("../../input/day1.sample");

pub struct Solution {}
impl super::utils::Solution for Solution {
    fn new() -> impl super::utils::Solution {
        Self {}
    }

    fn run_sample(&mut self) {
        run("sample", SAMPLE)
    }

    fn run_full(&mut self) {
        run("full", FULL)
    }
}

fn run(label: &'static str, data: &[u8]) {
    info!("{} start parsing", label);
    let mut left: ArrayVec<_, 1024> = ArrayVec::new();
    let mut right: ArrayVec<_, 1024> = ArrayVec::new();

    let mut it = nom::combinator::iterator(data, parse_line);
    for (a, b) in &mut it {
        debug!("{} a:{} b:{}", label, a, b);
        left.push(a);
        right.push(b);
    }
    if it.finish().is_err() {
        info!("{} error", label);
    } else {
        info!("{} processing", label);
        left.as_mut().sort_unstable();
        right.as_mut().sort_unstable();

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

fn parse_line(input: &[u8]) -> IResult<&[u8], (ResultType, ResultType)> {
    map_res(
        nom::sequence::tuple((
            integer::<ResultType>,
            whitespace,
            integer::<ResultType>,
            newline,
        )),
        |(lhs, _, rhs, _)| Ok::<_, &[u8]>((lhs, rhs)),
    )(input)
}
