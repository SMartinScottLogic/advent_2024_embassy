use arrayvec::ArrayVec;
use defmt::{error, info};

use nom::combinator::map_res;
use nom::combinator::{iterator, opt};
use nom::IResult;
use scapegoat::SgSet;

use super::utils::parse::newline;
use super::utils::parse::non_newline;

type ResultType = u64;

const FULL: &[u8] = include_bytes!("../../input/day10.full");
const SAMPLE: &[u8] = include_bytes!("../../input/day10.sample");

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
    let mut grid = ArrayVec::<&[u8], 64>::new();
    let mut it = iterator(data, grid_line);
    for line in &mut it {
        grid.push(line);
        if line.is_empty() {
            info!("Manual abort");
            break;
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
    info!("{} read {} rows", label, grid.len());
    info!("{} start processing", label);
    let mut step1_answer = 0;
    let mut step2_answer = 0;
    let max_y = (grid.len() - 1) as isize;
    let max_x = (grid.get(0).unwrap().len() - 1) as isize;

    for (y, r) in grid.iter().enumerate() {
        for (x, c) in r.iter().enumerate() {
            if *c == b'0' {
                let (reached, count) =
                    get_reachable::<64, 64>(&grid, x as isize, y as isize, max_x, max_y, *c);
                step1_answer += reached.len() as ResultType;
                step2_answer += count;
            }
        }
    }

    info!("{} part1 answer: {}", label, step1_answer);
    info!("{} part2 answer: {}", label, step2_answer);
}

fn grid_line(input: &[u8]) -> IResult<&[u8], &[u8]> {
    map_res(
        nom::sequence::tuple((non_newline, opt(newline))),
        |(line, _)| Ok::<_, &[u8]>(line),
    )(input)
}

fn get_reachable<const C: usize, const N: usize>(
    grid: &ArrayVec<&[u8], C>,
    x: isize,
    y: isize,
    max_x: isize,
    max_y: isize,
    c: u8,
) -> (SgSet<(isize, isize), N>, usize) {
    let mut reachable = SgSet::new();
    let mut total = 0;
    if c == b'9' {
        reachable.insert((x, y));
        return (reachable, 1);
    }
    for (dx, dy) in [(0, -1), (0, 1), (1, 0), (-1, 0)] {
        let nx = x + dx;
        let ny = y + dy;
        match get(grid, nx, ny, max_x, max_y) {
            Some(nc) if (*nc - b'0') == (c - b'0') + 1 => {
                let (a, b) = get_reachable::<C, N>(grid, nx, ny, max_x, max_y, *nc);
                for r in a {
                    reachable.insert(r);
                }
                total += b;
            }
            _ => {}
        }
    }
    (reachable, total)
}

fn get<'a, const C: usize>(
    grid: &'a ArrayVec<&[u8], C>,
    x: isize,
    y: isize,
    max_x: isize,
    max_y: isize,
) -> Option<&'a u8> {
    if x < 0 || x > max_x || y < 0 || y > max_y {
        None
    } else {
        grid.get(y as usize).and_then(|r| r.get(x as usize))
        //.inspect(|v| info!("valid read from ({},{}) => {}", x, y, v))
    }
}
