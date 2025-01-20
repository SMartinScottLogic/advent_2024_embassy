use defmt::{debug, info};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take;
use nom::bytes::complete::{take_while, take_while1};
use nom::character::complete::digit1;
use nom::combinator::iterator;
use nom::combinator::map_res;
use nom::IResult;

use crate::aoc::utils::FixedVec;

use super::utils::parse::{newline, non_newline};

pub type ResultType = u64;

const FULL: &[u8] = include_bytes!("../../input/day4.full");
const SAMPLE: &[u8] = include_bytes!("../../input/day4.sample");

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

    let mut grid = FixedVec::<&[u8], 200>::new();
    let mut it = iterator(data, grid_line);
    for line in &mut it {
        grid.push(line);
        if line.is_empty() {
            info!("Manual abort");
            break;
        }
    }
    info!("{} lines: {}", label, grid.len());
}

fn grid_line(input: &[u8]) -> IResult<&[u8], &[u8]> {
    map_res(nom::sequence::tuple((non_newline, newline)), |(line, _)| {
        Ok::<_, &[u8]>(line)
    })(input)
}
