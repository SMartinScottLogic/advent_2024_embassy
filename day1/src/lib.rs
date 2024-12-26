#![no_std]
extern crate alloc;
extern crate core;

use core::num::ParseIntError;

use alloc::{collections::btree_map::BTreeMap, vec::Vec};
use log::debug;
use utils::Solution as _;

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    left: Vec<ResultType>,
    right: Vec<ResultType>,
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
        let (a, b) = match it
            .next()
            .and_then(|v| it.next().and_then(|v2| Some((v, v2))))
        {
            Some((a, b)) => (a, b),
            None => panic!("Cannot parse {}", line),
        };
        let a = a.parse()?;
        let b = b.parse()?;
        self.left.push(a);
        self.right.push(b);
        Ok(())
    }

    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Result<Self::ResultType, utils::Error> {
        let mut left = self.left.clone();
        left.sort();
        let mut right = self.right.clone();
        right.sort();
        for (a, b) in left.iter().zip(right.iter()) {
            debug!("sorted {} vs {}", a, b);
        }
        let answer = left
            .iter()
            .zip(right.iter())
            .map(|(a, b)| Self::distance(a, b))
            .sum();
        // Implement for problem
        Ok(answer)
    }

    fn answer_part2(&self, _is_full: bool) -> Result<Self::ResultType, utils::Error> {
        let right_count = self.right.iter().fold(BTreeMap::new(), |mut acc, value| {
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
