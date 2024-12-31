#![no_std]
extern crate core;

use core::num::ParseIntError;

use log::debug;
use scapegoat::SgMap;
use utils::Solution as _;
use utils::collections::FixedVec;

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    left: FixedVec<ResultType, 1024>,  //Vec<ResultType>,
    right: FixedVec<ResultType, 1024>, //Vec<ResultType>,
}
impl Solution {
    fn distance(a: &ResultType, b: &ResultType) -> ResultType {
        if a < b { b - a } else { a - b }
    }
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

    fn update_from_line(&mut self, _id: usize, line: &str) -> Result<(), Self::ParseError> {
        let mut it = line.split_whitespace();
        let (a, b) = match it.next().and_then(|v| it.next().map(|v2| (v, v2))) {
            Some((a, b)) => (a, b),
            None => panic!("Cannot parse {}", line),
        };
        let a = a.parse()?;
        let b = b.parse()?;
        self.left.push(a);
        self.right.push(b);
        Ok(())
    }

    fn analyse(&mut self, _is_full: bool) {
        self.left.sort();
        self.right.sort();
    }

    fn answer_part1(&self, _is_full: bool) -> Result<Self::ResultType, utils::Error> {
        for (a, b) in self.left.iter().zip(self.right.iter()) {
            debug!("sorted {} vs {}", a, b);
        }
        let answer = self
            .left
            .iter()
            .zip(self.right.iter())
            .map(|(a, b)| Self::distance(a, b))
            .sum();
        // Implement for problem
        Ok(answer)
    }

    fn answer_part2(&self, _is_full: bool) -> Result<Self::ResultType, utils::Error> {
        let right_count = self
            .right
            .iter()
            .fold(SgMap::<_, _, 1000>::new(), |mut acc, value| {
                let entry: &mut ResultType = acc.entry(*value).or_default();
                *entry += 1;
                acc
            });

        let answer = self
            .left
            .iter()
            .map(|v| v * right_count.get(v).cloned().unwrap_or_default())
            .sum();

        // Implement for problem
        Ok(answer)
    }
}
