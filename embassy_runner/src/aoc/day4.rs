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

pub type ResultType = u64;

const FULL: &'static [u8] = include_bytes!("../../../input/day4.full");
const SAMPLE: &'static [u8] = include_bytes!("../../../input/day4.sample");

pub fn run_sample() {
    run("sample", SAMPLE)
}

pub fn run_full() {
    run("full", FULL)
}

fn run(label: &'static str, data: &[u8]) {
    info!("{} start parsing", label);

    let mut grid = FixedVec::<&[u8], 200>::new();
    let mut it = iterator(data, grid_line);
    for line in &mut it {
        grid.push(line);
        if line.len() == 0 {
            info!("Manual abort");
            break;
        }
    }
    info!("{} lines: {}", label, grid.len());
}

fn newline(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while(|c| c == b'\n' || c == b'\r')(input)
}
fn non_newline(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while1(|c| c != b'\n' && c != b'\r')(input)
}

fn grid_line(input: &[u8]) -> IResult<&[u8], &[u8]> {
    map_res(nom::sequence::tuple((non_newline, newline)), |(line, _)| {
        Ok::<_, &[u8]>(line)
    })(input)
}
