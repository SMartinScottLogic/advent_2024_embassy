#![no_std]
#![allow(unused_imports)]
#![feature(let_chains)]

//extern crate alloc;
extern crate core;

use core::num::ParseIntError;

//use alloc::borrow::ToOwned;
use log::{debug, info};
use utils::collections::FixedVec;
use utils::grid::FixedGrid;
use utils::point::Direction;
use utils::{Solution as _, point::Point};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    grid: FixedGrid<char, 150>,
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

    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Result<Self::ResultType, utils::Error> {
        let mut total = 0;
        let range = self.grid.dimensions();
        for sy in range.y.clone() {
            for sx in range.x.clone() {
                let pos = Point::new(sx, sy);
                if let Some('X') = self.grid.get(&pos) {
                    for delta in Direction::iter() {
                        let pos = Point::new(sx, sy) + delta;
                        let mut s = FixedVec::<char, 4>::new();
                        s.push('X');
                        if self.walk(pos, &delta, &mut s, "XMAS") {
                            total += 1;
                            debug!("found {sx}, {sy}, {total}");
                        }
                    }
                }
            }
        }
        // Implement for problem
        Ok(total)
    }

    fn answer_part2(&self, _is_full: bool) -> Result<Self::ResultType, utils::Error> {
        let mut total = 0;
        let range = self.grid.dimensions();
        for sy in range.y.clone() {
            for sx in range.x.clone() {
                let start = Point::new(sx, sy);
                if let Some('M') = self.grid.get(&start) {
                    for (delta, next_deltas) in [
                        (Direction::NE, [Direction::SE, Direction::NW]),
                        (Direction::SE, [Direction::NE, Direction::SW]),
                        (Direction::SW, [Direction::SE, Direction::NW]),
                        (Direction::NW, [Direction::NE, Direction::SW]),
                    ] {
                        let mut s = FixedVec::<char, 4>::new();
                        s.push('M');
                        if self.walk(start + delta, &delta, &mut s, "MAS") {
                            for next_delta in next_deltas {
                                let new_start = start + delta - next_delta;
                                if let Some('M') = self.grid.get(&new_start) {
                                    s.clear();
                                    s.push('M');
                                    if self.walk(new_start + next_delta, &next_delta, &mut s, "MAS")
                                    {
                                        total += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        // Implement for problem
        Ok(total / 2)
    }
}

impl Solution {
    fn walk(
        &self,
        pos: Point<isize>,
        delta: &Direction,
        s: &mut FixedVec<char, 4>,
        target: &str,
    ) -> bool {
        if let Some(c) = self.grid.get(&pos)
            && *c == target.chars().nth(s.len()).unwrap()
        {
            if target.len() == s.len() + 1 {
                true
            } else {
                s.push(*c);
                self.walk(pos + delta, delta, s, target)
            }
        } else {
            false
        }
    }
}
