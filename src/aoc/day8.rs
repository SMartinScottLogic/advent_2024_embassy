use arrayvec::ArrayVec;
use defmt::{error, info};

use nom::combinator::map_res;
use nom::combinator::{iterator, opt};
use nom::IResult;
use scapegoat::SgSet;

use super::utils::parse::newline;
use super::utils::parse::non_newline;

const FULL: &[u8] = include_bytes!("../../input/day8.full");
const SAMPLE: &[u8] = include_bytes!("../../input/day8.sample");

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
    let step1_answer = analyse_part1(&grid);
    info!("{} part1 answer: {}", label, step1_answer);
    let step2_answer = analyse_part2(&grid);
    info!("{} part2 answer: {}", label, step2_answer);
}

fn grid_line(input: &[u8]) -> IResult<&[u8], &[u8]> {
    map_res(
        nom::sequence::tuple((non_newline, opt(newline))),
        |(line, _)| Ok::<_, &[u8]>(line),
    )(input)
}

fn analyse_part1<const C: usize>(grid: &ArrayVec<&[u8], C>) -> usize {
    let mut antinodes = SgSet::<(isize, isize), 1024>::new();
    let min_y = 0_isize;
    let max_y = (grid.len() - 1) as isize;
    let min_x = 0_isize;
    let max_x = (grid.first().unwrap().len() - 1) as isize;

    for (p1_y, p1_r) in grid.iter().enumerate() {
        for (p1_x, p1_c) in p1_r.iter().enumerate() {
            for (p2_y, p2_r) in grid.iter().enumerate() {
                for (p2_x, p2_c) in p2_r.iter().enumerate() {
                    let p1x = p1_x as isize;
                    let p1y = p1_y as isize;
                    let p2x = p2_x as isize;
                    let p2y = p2_y as isize;

                    if p1x == p2x && p1y == p2y {
                        continue;
                    }
                    if p1_c != p2_c {
                        continue;
                    }
                    if *p1_c == b'.' {
                        continue;
                    }
                    let dx = p1x - p2x;
                    let dy = p1y - p2y;
                    for (x, y) in [
                        (p1x + dx, p1y + dy),
                        (p2x + dx, p2y + dy),
                        (p1x - dx, p1y - dy),
                        (p2x - dx, p2y - dy),
                    ] {
                        if x != p1x
                            && y != p1y
                            && x != p2x
                            && y != p2y
                            && x >= min_x
                            && x <= max_x
                            && y >= min_y
                            && y <= max_y
                        {
                            antinodes.insert((x, y));
                        }
                    }
                }
            }
        }
    }
    antinodes.len()
}

fn analyse_part2<const C: usize>(grid: &ArrayVec<&[u8], C>) -> usize {
    let mut antinodes = SgSet::<(isize, isize), 1024>::new();
    let min_y = 0_isize;
    let max_y = (grid.len() - 1) as isize;
    let min_x = 0_isize;
    let max_x = (grid.first().unwrap().len() - 1) as isize;

    for (p1_y, p1_r) in grid.iter().enumerate() {
        for (p1_x, p1_c) in p1_r.iter().enumerate() {
            for (p2_y, p2_r) in grid.iter().enumerate() {
                for (p2_x, p2_c) in p2_r.iter().enumerate() {
                    let p1x = p1_x as isize;
                    let p1y = p1_y as isize;
                    let p2x = p2_x as isize;
                    let p2y = p2_y as isize;

                    if p1x == p2x && p1y == p2y {
                        continue;
                    }
                    if p1_c != p2_c {
                        continue;
                    }
                    if *p1_c == b'.' {
                        continue;
                    }
                    let dx = p1x - p2x;
                    let dy = p1y - p2y;

                    let mut x = p1x;
                    let mut y = p1y;
                    loop {
                        if x < min_x || x > max_x || y < min_y || y > max_y {
                            break;
                        }
                        antinodes.insert((x, y));
                        x -= dx;
                        y -= dy;
                    }
                    let mut x = p1x;
                    let mut y = p1y;
                    loop {
                        if x < min_x || x > max_x || y < min_y || y > max_y {
                            break;
                        }
                        antinodes.insert((x, y));
                        x += dx;
                        y += dy;
                    }
                }
            }
        }
    }
    antinodes.len()
}
