use core::{convert::Infallible, num::ParseIntError};

use alloc::{collections::btree_map::BTreeMap, vec::Vec};
use defmt::debug;

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    left: Vec<ResultType>,
    right: Vec<ResultType>,
}
impl Solution {
    fn distance(a: &ResultType, b: &ResultType) -> ResultType {
        if a < b {
            b - a
        } else {
            a - b
        }
    }
}
impl TryFrom<&str> for Solution {
    type Error = ParseIntError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for line in value.lines() {
            if let Some((a, b)) = line.split_once(' ') {
                let a = a.parse()?;
                let b = b.parse()?;
                solution.left.push(a);
                solution.right.push(b);
            } else {
                panic!("Cannot parse {}", value);
            }
        }
        Ok(solution)
    }
}
impl crate::aoc::utils::Solution for Solution {
    type Result = Result<ResultType, Infallible>;

    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
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

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
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
