#![no_std]
#![allow(unused_imports)]

extern crate core;

use core::cmp::Ordering;
use core::num::ParseIntError;

use log::{debug, info};
use utils::{Solution as _, collections::FixedVec};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    reports: FixedVec<FixedVec<ResultType, 10>, 1024>,
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
    fn update_from_line(&mut self, _id: usize, line: &str) -> Result<(), Self::ParseError> {
        let levels =
            line.split_whitespace()
                .flat_map(|v| v.parse())
                .fold(FixedVec::new(), |mut acc, v| {
                    acc.push(v);
                    acc
                });
        self.reports.push(levels);
        Ok(())
    }

    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Result<Self::ResultType, utils::Error> {
        let answer = self
            .reports
            .iter()
            .filter(|v| Self::is_safe_part1(v))
            .count();
        Ok(answer as ResultType)
    }

    fn answer_part2(&self, _is_full: bool) -> Result<Self::ResultType, utils::Error> {
        // Implement for problem
        let answer = self
            .reports
            .iter()
            .filter(|v| {
                if Self::is_safe_part1(v) {
                    true
                } else {
                    for skip in 0..v.len() {
                        if Self::is_safe_part2(v, skip) {
                            return true;
                        }
                    }
                    false
                }
            })
            .count();
        Ok(answer as ResultType)
    }
}

impl Solution {
    fn is_safe_part1<const C: usize>(report: &FixedVec<ResultType, C>) -> bool {
        let mut dir = Ordering::Equal;
        let mut last = 0;
        for (i, cur) in report.iter().enumerate() {
            if i == 0 {
                last = *cur;
                continue;
            }
            if last == *cur {
                return false;
            }
            if last > *cur {
                if dir == Ordering::Less {
                    return false;
                }
                if last - *cur > 3 {
                    return false;
                }
                dir = Ordering::Greater;
            }
            if last < *cur {
                if dir == Ordering::Greater {
                    return false;
                }
                if *cur - last > 3 {
                    return false;
                }
                dir = Ordering::Less;
            }
            last = *cur;
        }
        debug!("safe: {:?}", report);
        true
    }

    fn is_safe_part2<const C: usize>(report: &FixedVec<ResultType, C>, skip: usize) -> bool {
        let mut dir = Ordering::Equal;
        let mut last = None;
        for (i, cur) in report.iter().enumerate() {
            if i == skip {
                continue;
            }
            if last.is_none() {
                last = Some(*cur);
                continue;
            }
            if last.unwrap() == *cur {
                return false;
            }
            if last.unwrap() > *cur {
                if dir == Ordering::Less {
                    return false;
                }
                if last.unwrap() - *cur > 3 {
                    return false;
                }
                dir = Ordering::Greater;
            }
            if last.unwrap() < *cur {
                if dir == Ordering::Greater {
                    return false;
                }
                if *cur - last.unwrap() > 3 {
                    return false;
                }
                dir = Ordering::Less;
            }
            last = Some(*cur);
        }
        debug!("safe: {:?}", report);
        true
    }
}
