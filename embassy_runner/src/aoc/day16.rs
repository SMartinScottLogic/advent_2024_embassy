use defmt::{debug, error, info};

use nom::bytes::complete::tag;
use nom::bytes::complete::take_while;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::multi::fold_many1;
use nom::IResult;

use crate::aoc::utils::FixedVec;

pub type ResultType = u64;

const FULL: &'static [u8] = include_bytes!("../../../input/day16.full");
const SAMPLE: &'static [u8] = include_bytes!("../../../input/day16.sample");

#[derive(Default, Clone, PartialEq)]
pub enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
    #[default]
    None,
}
impl defmt::Format for Direction {
    fn format(&self, fmt: defmt::Formatter) {
        let d = match self {
            Direction::N => "Direction::N",
            Direction::NE => "Direction::NE",
            Direction::E => "Direction::E",
            Direction::SE => "Direction::SE",
            Direction::S => "Direction::S",
            Direction::SW => "Direction::SW",
            Direction::W => "Direction::W",
            Direction::NW => "Direction::NW",
            Direction::None => "Direction::None",
        };
        defmt::write!(fmt, "{}", d);
    }
}

pub fn run_sample() {
    run("sample", SAMPLE)
}

pub fn run_full() {
    run("full", FULL)
}

fn run(label: &'static str, data: &[u8]) {
    let mut data = data;
    let mut row = 0;
    let mut grid = FixedVec::<&[u8], 200>::new();
    loop {
        if let Ok((r, gridline)) = grid_line(data) {
            debug!("{} {}: {}", label, row, gridline.len());
            grid.push(gridline);
            data = r;
        } else {
            error!("{} {}: Error ({})", label, row, data[0]);
            break;
        }

        if data.is_empty() {
            break;
        }
        row += 1;
    }
    info!("{}: {} lines", label, grid.len());
    let mut start = (Direction::E, 0, 0);
    let mut end = (0, 0);

    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            match *c {
                b'S' => start = (Direction::E, x, y),
                b'E' => end = (x, y),
                _ => {}
            }
        }
    }
    info!("{} start @ {}, end @ {}", label, start, end);
    let answer = if let Some((_route, cost)) = pathfinding::directed::astar::astar(
        &start,
        |p| successors(&grid, p.0, p.1, p.2),
        |p| heuristic(p.0, p.1, p.2, &end),
        |p| success(p.0, p.1, p.2, &end),
    ) {
        cost
    } else {
        0
    };
    // Implement for problem
    info!("Part1 answer: {}", answer);
}

fn success(_facing: Direction, x: usize, y: usize, end: &(usize, usize)) -> bool {
    end.0 == x && end.1 == y
}
fn heuristic(_facing: Direction, x: usize, y: usize, end: &(usize, usize)) -> ResultType {
    (end.0.abs_diff(x) + end.1.abs_diff(y)) as ResultType
}
fn successors(
    grid: &FixedVec<&[u8], 200>,
    facing: Direction,
    x: usize,
    y: usize,
) -> impl Iterator<Item = ((Direction, usize, usize), ResultType)> {
    match facing {
        Direction::N => [
            ((Direction::E, x, y), 1000),
            ((Direction::W, x, y), 1000),
            if y > 0 {
                ((Direction::N, x, y - 1), 1)
            } else {
                ((Direction::None, x, y), 1)
            },
        ],
        Direction::S => [
            ((Direction::E, x, y), 1000),
            ((Direction::W, x, y), 1000),
            ((Direction::S, x, y + 1), 1),
        ],
        Direction::E => [
            ((Direction::N, x, y), 1000),
            ((Direction::S, x, y), 1000),
            ((Direction::E, x + 1, y), 1),
        ],
        Direction::W => [
            ((Direction::N, x, y), 1000),
            ((Direction::S, x, y), 1000),
            if x > 0 {
                ((Direction::W, x - 1, y), 1)
            } else {
                ((Direction::None, x, y), 1)
            },
        ],
        _ => panic!(),
    }
    .into_iter()
    .filter(|&((ref facing, x, y), _c)| {
        facing != &Direction::None
            && *grid.get(y).and_then(|row| row.get(x)).unwrap_or(&b'#') != b'#'
    })
    .collect::<FixedVec<_, 3>>()
    .into_iter()
}

fn newline(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while(|c| c == b'\n' || c == b'\r')(input)
}
fn non_newline(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while(|c| c != b'\n' && c != b'\r')(input)
}

fn grid_line(input: &[u8]) -> IResult<&[u8], &[u8]> {
    map_res(nom::sequence::tuple((non_newline, newline)), |(line, _)| {
        Ok::<_, &[u8]>(line)
    })(input)
}
