use core::convert::Infallible;

use crate::aoc::utils;
#[allow(unused_imports)]
use defmt::{debug, info};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {}

impl TryFrom<&str> for Solution {
    type Error = Infallible;

    #[allow(unused_variables, unused_mut)]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        todo!()
    }
}
// impl utils::Solution for Solution {
//     type Result = Result<ResultType, Infallible>;

//     fn analyse(&mut self, _is_full: bool) {}

//     fn answer_part1(&self, _is_full: bool) -> Self::Result {
//         // Implement for problem
//         Ok(0)
//     }

//     fn answer_part2(&self, _is_full: bool) -> Self::Result {
//         // Implement for problem
//         Ok(0)
//     }
// }
