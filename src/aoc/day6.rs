use defmt::{debug, error, info};

use nom::bytes::complete::tag;
use nom::combinator::iterator;
use nom::combinator::map_res;
use nom::IResult;

use crate::aoc::utils::FixedVec;

use super::utils::parse::integer;
use super::utils::parse::list_number;
use super::utils::parse::newline;
use super::utils::parse::non_newline;

type ResultType = u64;

const FULL: &[u8] = include_bytes!("../../input/day6.full");
const SAMPLE: &[u8] = include_bytes!("../../input/day6.sample");

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
    info!("{} read {} gridlines", label, grid.len());
    info!("{} start processing", label);
}

fn grid_line(input: &[u8]) -> IResult<&[u8], &[u8]> {
    map_res(nom::sequence::tuple((non_newline, newline)), |(line, _)| {
        Ok::<_, &[u8]>(line)
    })(input)
}
