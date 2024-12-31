#![no_std]
#![allow(unused_imports)]
extern crate core;

use core::num::ParseIntError;

use log::{debug, info};
use utils::Solution as _;

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {}

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
    fn update_from_line(&mut self, _id: usize, line: &str) -> Result<(), Self::ParseError> {
        todo!()
    }

    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Result<Self::ResultType, utils::Error> {
        // Implement for problem
        Ok(0)
    }

    fn answer_part2(&self, _is_full: bool) -> Result<Self::ResultType, utils::Error> {
        // Implement for problem
        Ok(0)
    }
}
