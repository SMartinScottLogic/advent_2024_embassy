#![no_std]
#![allow(unused_imports)]
extern crate core;

use core::cmp::Ordering;
use core::num::ParseIntError;

use log::{debug, info};
use scapegoat::SgSet;
use utils::{Solution as _, collections::FixedVec};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    rules: SgSet<(ResultType, ResultType), 2048>,
    updates: FixedVec<FixedVec<ResultType, 32>, 512>,
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
        let line = line.trim();
        if line.is_empty() {
            Ok(())
        } else if let Some((a, b)) = line.split_once('|') {
            let a = a.parse().unwrap();
            let b = b.parse().unwrap();
            self.rules.insert((a, b));
            Ok(())
        } else {
            let update =
                line.split(',')
                    .map(|v| v.parse())
                    .flatten()
                    .fold(FixedVec::new(), |mut acc, v| {
                        acc.push(v);
                        acc
                    });
            self.updates.push(update);
            Ok(())
        }
    }

    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Result<Self::ResultType, utils::Error> {
        let mut total = 0;
        for update in self.updates.iter() {
            let correct = self.is_correct(update.as_ref());
            debug!("correct: {:?}", correct);
            if correct {
                let mid = update.get(update.len() / 2).unwrap();
                debug!("mid: {}, {:?}", mid, update);
                total += mid;
            }
        }
        // Implement for problem
        Ok(total)
    }

    fn answer_part2(&self, _is_full: bool) -> Result<Self::ResultType, utils::Error> {
        let mut total = 0;
        for update in self.updates.iter() {
            let correct = self.is_correct(update.as_ref());
            debug!("correct: {:?}", correct);
            if !correct {
                let mut fixed = (*update).clone();
                self.fix(fixed.as_mut_ref());
                let mid = fixed.get(update.len() / 2).unwrap();
                debug!("mid: {}, {:?} -> {:?}", mid, update, fixed);
                total += mid;
            }
        }
        // Implement for problem
        Ok(total)
    }
}

impl Solution {
    fn is_correct(&self, update: &[ResultType]) -> bool {
        for (i, page) in update.iter().enumerate() {
            for j in 0..i {
                let probe = update.get(j).unwrap();
                if self.rules.contains(&(*page, *probe)) {
                    return false;
                }
            }
        }
        true
    }

    fn fix(&self, arr: &mut [ResultType]) {
        arr.sort_by(|a, b| {
            if self.rules.contains(&(*b, *a)) {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });
    }
}
