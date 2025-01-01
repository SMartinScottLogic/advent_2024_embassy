#![no_std]
#![allow(unused_imports)]
#![feature(generic_const_exprs)]

extern crate core;

use core::num::ParseIntError;

use log::{debug, info};
use scapegoat::SgSet;
use utils::{Solution as _, grid::FixedGrid, point::Point};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    grid: FixedGrid<u64, 50>,
    part1_answer: ResultType,
    part2_answer: ResultType,
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
            self.grid
                .set(&Point::new(x, y), c.to_digit(10).unwrap_or(10).into());
        }
        Ok(())
    }

    fn analyse(&mut self, _is_full: bool) {
        let mut part1 = 0;
        let mut part2 = 0;
        for (p, c) in self.grid.iter() {
            if *c == 0 {
                let (reached, count) = self.get_reachable(&p, *c);
                part1 += reached.len() as ResultType;
                part2 += count;
            }
        }
        self.part1_answer = part1;
        self.part2_answer = part2;
    }

    fn answer_part1(&self, _is_full: bool) -> Result<Self::ResultType, utils::Error> {
        // Implement for problem
        Ok(self.part1_answer)
    }

    fn answer_part2(&self, _is_full: bool) -> Result<Self::ResultType, utils::Error> {
        // Implement for problem
        Ok(self.part2_answer)
    }
}

impl Solution {
    fn get_reachable(
        &self,
        cur_position: &(isize, isize),
        c: ResultType,
    ) -> (SgSet<(isize, isize), 1_000>, ResultType) {
        let mut reachable: SgSet<_, 1_000> = SgSet::new();
        let mut total = 0;
        if c == 9 {
            reachable.insert(*cur_position);
            return (reachable, 1);
        }
        let cur_position = Point::new(cur_position.0, cur_position.1);
        for next_pos in cur_position.cardinal() {
            match self.grid.get(&next_pos) {
                Some(nc) if *nc == c + 1 => {
                    let (a, b) = self.get_reachable(&(next_pos.x(), next_pos.y()), *nc);
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
}
