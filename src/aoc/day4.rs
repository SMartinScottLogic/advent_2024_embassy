use crate::{debug, info};
use arrayvec::ArrayVec;

use nom::combinator::iterator;
use nom::combinator::map_res;
use nom::IResult;

use super::utils::parse::{newline, non_newline};

const FULL: &[u8] = include_bytes!("../../input/day4.full");
const SAMPLE: &[u8] = include_bytes!("../../input/day4.sample");

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

    let mut grid = ArrayVec::<&[u8], 200>::new();
    let mut it = iterator(data, grid_line);
    for line in &mut it {
        grid.push(line);
        if line.is_empty() {
            info!("Manual abort");
            break;
        }
    }
    info!("{} start processing", label);
    let all_directions = [
        (0, -1),  //Direction::N,
        (0, 1),   //Direction::S,
        (1, 0),   //Direction::E,
        (-1, 0),  //Direction::W,
        (1, -1),  //Direction::NE,
        (-1, 1),  //Direction::SW,
        (1, 1),   //Direction::SE,
        (-1, -1), //Direction::NW,
    ];
    let mut step1_answer = 0;
    for (sy, row) in grid.iter().enumerate() {
        for (sx, c) in row.iter().enumerate() {
            if c == &b'X' {
                for (dx, dy) in all_directions.iter() {
                    if walk(&grid, sx as isize, sy as isize, *dx, *dy, b"XMAS") {
                        step1_answer += 1;
                        debug!("({},{}): {} found", sx, sy, step1_answer);
                    }
                }
            }
        }
    }
    info!("{} step1 answer = {}", label, step1_answer);
    let mut step2_answer = 0;
    for (sy, row) in grid.iter().enumerate() {
        for (sx, c) in row.iter().enumerate() {
            if c == &b'M' {
                for ((dx, dy), next_deltas) in [
                    ((1, -1), [(1, 1), (-1, -1)]), // Direction::NE + {Direction::SE || Direction::NW}
                    ((1, 1), [(1, -1), (-1, 1)]), // Direction::SE + {Direction::NE || Direction::SW}
                    ((-1, 1), [(1, 1), (-1, -1)]), // Direction::SW + {Direction::SE || Direction::NW}
                    ((-1, -1), [(1, -1), (-1, 1)]), // Direction::NW + {Direction::NE || Direction::SW}
                ] {
                    if walk(&grid, sx as isize, sy as isize, dx, dy, b"MAS") {
                        for (nd_x, nd_y) in next_deltas {
                            let ns_x = sx as isize + dx - nd_x;
                            let ns_y = sy as isize + dy - nd_y;
                            if walk(&grid, ns_x, ns_y, nd_x, nd_y, b"MAS") {
                                step2_answer += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    // Implement for problem
    info!("{} step2 answer = {}", label, step2_answer / 2);
}

fn walk(
    grid: &ArrayVec<&[u8], 200>,
    sx: isize,
    sy: isize,
    dx: isize,
    dy: isize,
    needle: &[u8],
) -> bool {
    let mut x = sx;
    let mut y = sy;

    for n in needle {
        if x < 0 || y < 0 {
            return false;
        }
        if let Some(v) = grid.get(y as usize).and_then(|row| row.get(x as usize)) {
            if v != n {
                return false;
            }
        } else {
            return false;
        }
        x += dx;
        y += dy;
    }
    true
}

fn grid_line(input: &[u8]) -> IResult<&[u8], &[u8]> {
    map_res(nom::sequence::tuple((non_newline, newline)), |(line, _)| {
        Ok::<_, &[u8]>(line)
    })(input)
}
