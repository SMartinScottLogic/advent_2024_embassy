#![no_std]
#![allow(unused_imports)]
#![feature(generic_const_exprs)]
extern crate core;

use core::num::ParseIntError;

use log::{debug, info};
use utils::{
    Solution as _,
    grid::FixedGrid,
    point::{Direction, Point},
};

pub type ResultType = u64;

#[derive(Debug)]
enum Decision {
    Step,
    Turn,
}

#[derive(Debug, Default)]
pub struct Solution {
    grid: FixedGrid<char, 150>,
    guard_pos: Point<isize>,
}

impl TryFrom<&str> for Solution {
    type Error = ParseIntError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (id, line) in value.lines().enumerate() {
            solution.update_from_line(id, line)?;
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type ResultType = ResultType;
    type ParseError = ParseIntError;

    #[allow(unused_variables)]
    fn update_from_line(&mut self, y: usize, line: &str) -> Result<(), Self::ParseError> {
        for (x, c) in line.chars().enumerate() {
            self.grid.set(&Point::new(x as isize, y as isize), c);
        }
        Ok(())
    }

    fn analyse(&mut self, _is_full: bool) {
        for y in 0..150 {
            for x in 0..150 {
                let position = Point::new(x as isize, y as isize);
                if let Some(c) = self.grid.get(&position) {
                    if *c == '^' {
                        self.guard_pos = position;
                    }
                }
            }
        }
    }

    fn answer_part1(&self, _is_full: bool) -> Result<Self::ResultType, utils::Error> {
        // Implement for problem
        let AnalyseResult(looped, visited, ..) = self.analyse(self.guard_pos, Direction::N, None);
        assert!(!looped);
        debug!("visited: {:?}", visited);
        let mut result = 0;
        for y in 0..150 {
            for x in 0..150 {
                let position = Point::new(x as isize, y as isize);
                if let Some(val) = visited.get(&position) {
                    debug!("visited ({},{}): {:?}", x, y, val);
                    if val[0].is_some() {
                        result += 1;
                    }
                }
            }
        }

        // Implement for problem
        Ok(result)
    }

    fn answer_part2(&self, _is_full: bool) -> Result<Self::ResultType, utils::Error> {
        // Implement for problem
        let mut count = 0;
        let AnalyseResult(_, _visited, first_visited) =
            self.analyse(self.guard_pos, Direction::N, None);
        debug!("first visits: {:?}", first_visited);
        for y in 0..150 {
            for x in 0..150 {
                let position = Point::new(x, y);
                if let Some(direction) = Self::get(&first_visited, position).unwrap() {
                    debug!("test {:?}", position);
                    let guard_pos = match direction {
                        Direction::N => position.south(),
                        Direction::E => position.west(),
                        Direction::S => position.north(),
                        Direction::W => position.east(),
                        _ => panic!(),
                    };
                    if self.analyse(guard_pos, *direction, Some(position)).0 {
                        count += 1;
                    }
                }
            }
        }
        // for (i, (position, ..)) in visited.enumerate_row_major().enumerate() {
        //     let position = Point::new(position.0 as isize, position.1 as isize);
        //     debug!(i, ?position, "test");
        //     if let Some(direction) = Self::get(&first_visited, position).unwrap() {
        //         let guard_pos = match direction {
        //             Direction::N => position.south(),
        //             Direction::E => position.west(),
        //             Direction::S => position.north(),
        //             Direction::W => position.east(),
        //             _ => panic!(),
        //         };
        //         if self.analyse(guard_pos, *direction, Some(position)).0 {
        //             count += 1;
        //         }
        //     }
        // }
        Ok(count)
    }
}

struct AnalyseResult(
    bool,
    FixedGrid<[Option<Direction>; 4], 150>,
    FixedGrid<Option<Direction>, 150>,
);

impl Solution {
    fn analyse(
        &self,
        mut guard_pos: Point<isize>,
        mut direction: Direction,
        additional_obstacle: Option<Point<isize>>,
    ) -> AnalyseResult {
        // Implement for problem
        let mut steps = 0;
        let mut visited = FixedGrid::filled_with([None; 4]);
        let mut first_visited = FixedGrid::filled_with(None);
        if matches!(additional_obstacle, Some(p) if p == guard_pos) {
            return AnalyseResult(false, visited, first_visited);
        }
        loop {
            if let Some(v) = Self::get_mut(&mut visited, guard_pos) {
                let mut found = false;
                let mut insert = None;
                for (i, v) in v.iter().enumerate() {
                    match v {
                        Some(d) if d == &direction => {
                            found = true;
                            break;
                        }
                        None if insert.is_none() => {
                            insert = Some(i);
                        }
                        _ => {}
                    }
                }
                if found {
                    return AnalyseResult(true, visited, first_visited);
                } else {
                    v[insert.unwrap()] = Some(direction);
                }
            }

            debug!("stage {}: {:?} -> {:?}", steps, guard_pos, direction);
            let front_pos = match direction {
                Direction::N => guard_pos.north(),
                Direction::E => guard_pos.east(),
                Direction::S => guard_pos.south(),
                Direction::W => guard_pos.west(),
                _ => panic!("unexpected direction {:?}", direction),
            };
            match match Self::get(&self.grid, front_pos) {
                _ if additional_obstacle.map(|p| front_pos == p).unwrap_or(false) => Decision::Turn,
                Some('.') => Decision::Step,
                Some('#') => Decision::Turn,
                // Guard can't stand in front of themselves
                Some('^') => Decision::Step,
                Some(c) if *c == char::default() => {
                    break AnalyseResult(false, visited, first_visited);
                }
                None => break AnalyseResult(false, visited, first_visited),
                Some(c) => panic!("Unknown entry in grid: {}", c),
            } {
                Decision::Step => {
                    steps += 1;
                    if let Some(e) = Self::get_mut(&mut first_visited, front_pos) {
                        if e.is_none() {
                            *e = Some(direction);
                        }
                    }
                    guard_pos = front_pos;
                }
                Decision::Turn => {
                    direction = match direction {
                        Direction::N => Direction::E,
                        Direction::E => Direction::S,
                        Direction::S => Direction::W,
                        Direction::W => Direction::N,
                        _ => panic!("unexpected direction {:?}", direction),
                    }
                }
            }
        }
    }

    fn get<T>(a: &FixedGrid<T, 150>, pos: Point<isize>) -> Option<&T> {
        if pos.x() < 0 || pos.y() < 0 {
            None
        } else {
            a.get(&pos)
        }
    }

    fn get_mut<T>(a: &mut FixedGrid<T, 150>, pos: Point<isize>) -> Option<&mut T> {
        if pos.x() < 0 || pos.y() < 0 {
            None
        } else {
            a.get_mut(&pos)
        }
    }
}
