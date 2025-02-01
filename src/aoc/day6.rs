use arrayvec::ArrayVec;
use defmt::{debug, info};

use nom::combinator::iterator;
use nom::combinator::map_res;
use nom::IResult;
use static_cell::StaticCell;

use super::utils::direction::Direction;
use super::utils::parse::newline;
use super::utils::parse::non_newline;

const FULL: &[u8] = include_bytes!("../../input/day6.full");
const SAMPLE: &[u8] = include_bytes!("../../input/day6.sample");

static SEEN: StaticCell<[u8; 40_000]> = StaticCell::new();
static SEEN_WITH_OBSTACLE: StaticCell<[u8; 40_000]> = StaticCell::new();

pub struct Solution {
    seen: &'static mut [u8; 40_000],
    seen_with_obstacle: &'static mut [u8; 40_000],
}
impl super::utils::Solution for Solution {
    fn new() -> impl super::utils::Solution {
        let seen = SEEN.init_with(|| [0; 40_000]);
        let seen_with_obstacle = SEEN_WITH_OBSTACLE.init_with(|| [0; 40_000]);
        Self {
            seen,
            seen_with_obstacle,
        }
    }

    fn run_sample(&mut self) {
        run("sample", SAMPLE, self.seen, self.seen_with_obstacle)
    }

    fn run_full(&mut self) {
        run("full", FULL, self.seen, self.seen_with_obstacle)
    }
}

fn run(
    label: &'static str,
    data: &[u8],
    seen: &mut [u8; 40_000],
    seen_with_obstacle: &mut [u8; 40_000],
) {
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
    info!("{} read {} gridlines", label, grid.len());
    info!("{} start processing", label);
    seen.fill(0);
    analyse(&grid, seen, None);
    let part1_answer = seen.iter().filter(|v| **v != 0).count();
    info!("{} part1 answer = {}", label, part1_answer);

    let mut part2_answer = 0;
    for y in 0..200 {
        for x in 0..200 {
            let idx = x + y * 200;
            match seen.get(idx) {
                Some(i) if *i != 0 => {
                    seen_with_obstacle.fill(0);
                    if analyse(&grid, seen_with_obstacle, Some((x as isize, y as isize))) {
                        part2_answer += 1;
                        debug!("{} loop @ {},{}", label, x, y);
                    }
                }
                _ => {}
            }
        }
    }
    info!("{} part2 answer = {}", label, part2_answer);
}

fn grid_line(input: &[u8]) -> IResult<&[u8], &[u8]> {
    map_res(nom::sequence::tuple((non_newline, newline)), |(line, _)| {
        Ok::<_, &[u8]>(line)
    })(input)
}

fn analyse<const C: usize, const N: usize>(
    grid: &ArrayVec<&[u8], C>,
    has_seen: &mut [u8; N],
    obstacle: Option<(isize, isize)>,
) -> bool {
    let (mut guard_x, mut guard_y, mut direction) = guard_startpos(grid);
    if matches!(obstacle, Some((x, y)) if x == guard_x && y == guard_y) {
        return false;
    }
    loop {
        debug!(
            "guard: {}, {}, seen {} positions",
            guard_x,
            guard_y,
            has_seen.len()
        );
        let d = match direction {
            Direction::N => 1,
            Direction::E => 2,
            Direction::S => 4,
            Direction::W => 8,
            _ => unreachable!(),
        };
        let idx = guard_x + guard_y * 200;
        match has_seen.get_mut(idx as usize) {
            Some(v) => {
                if *v & d != 0 {
                    return true;
                }
                *v |= d;
            }
            None => {
                has_seen[idx as usize] = d;
                //has_seen.insert((guard_x, guard_y), d);
            }
        };
        let (dx, dy, rotated_direction) = match direction {
            Direction::N => (0, -1, Direction::E),
            Direction::E => (1, 0, Direction::S),
            Direction::S => (0, 1, Direction::W),
            Direction::W => (-1, 0, Direction::N),
            _ => unreachable!(),
        };
        // let decision = match get(grid, guard_x + dx, guard_y + dy) {
        //     _ if obstacle
        //         .map(|(ox, oy)| guard_x + dx == ox && guard_y + dy == oy)
        //         .unwrap_or(false) =>
        //     {
        //         Decision::Turn
        //     }
        //     Some(b'.') | Some(b'^') => Decision::Step,
        //     Some(_) => Decision::Turn,
        //     None => Decision::Exit,
        // };
        // (guard_x, guard_y, direction) = match decision {
        //     Decision::Step => (guard_x + dx, guard_y + dy, direction),
        //     Decision::Turn => (guard_x, guard_y, rotated_direction),
        //     Decision::Exit => {
        //         // Doesn't exist - left grid, so didn't loop
        //         return false;
        //     }
        // }
        (guard_x, guard_y, direction) = match get(grid, guard_x + dx, guard_y + dy) {
            _ if obstacle
                .map(|(ox, oy)| guard_x + dx == ox && guard_y + dy == oy)
                .unwrap_or(false) =>
            {
                (guard_x, guard_y, rotated_direction)
            }
            Some(b'.') | Some(b'^') => (guard_x + dx, guard_y + dy, direction),
            Some(_) => (guard_x, guard_y, rotated_direction),
            None => {
                // Doesn't exist - left grid, so didn't loop
                return false;
            }
        }
    }
}

fn guard_startpos<const C: usize>(grid: &ArrayVec<&[u8], C>) -> (isize, isize, Direction) {
    for (y, r) in grid.iter().enumerate() {
        for (x, c) in r.iter().enumerate() {
            if c == &b'^' {
                return (x as isize, y as isize, Direction::N);
            }
        }
    }
    panic!("Cannot find the guard in the grid");
}

fn get<'a, const C: usize>(grid: &'a ArrayVec<&[u8], C>, x: isize, y: isize) -> Option<&'a u8> {
    if x < 0 || y < 0 {
        None
    } else {
        grid.get(y as usize).and_then(|r| r.get(x as usize))
    }
}
