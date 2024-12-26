#![no_std]
#![allow(unused_imports)]
extern crate alloc;
extern crate core;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::num::ParseIntError;

use log::{debug, info};
use utils::Solution as _;

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    inputs: Vec<String>,
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
        self.inputs.push(line.to_string());
        Ok(())
    }

    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Result<Self::ResultType, utils::Error> {
        let regex = Regex::new(r"mul\((?<a>\d{1,3}),(?<b>\d{1,3})\)").unwrap();
        let mut total = 0;
        for input in &self.inputs {
            for capture in regex.captures_iter(input) {
                let a: ResultType = capture.name("a").unwrap().as_str().parse().unwrap();
                let b: ResultType = capture.name("b").unwrap().as_str().parse().unwrap();
                total += a * b;
            }
        }
        Ok(total)
    }

    fn answer_part2(&self, _is_full: bool) -> Result<Self::ResultType, utils::Error> {
        let regex = Regex::new(r"(?<op>do|don't|mul)\(((?<a>\d{1,3}),(?<b>\d{1,3}))?\)").unwrap();
        let mut total = 0;
        let mut enable = true;
        for input in &self.inputs {
            for capture in regex.captures_iter(input) {
                let op = capture.name("op").unwrap().as_str();
                match op {
                    "do" => enable = true,
                    "don't" => enable = false,
                    "mul" => {
                        if enable {
                            let a: ResultType =
                                capture.name("a").unwrap().as_str().parse().unwrap();
                            let b: ResultType =
                                capture.name("b").unwrap().as_str().parse().unwrap();
                            total += a * b;
                        }
                    }
                    _ => panic!("unknown operator '{op}'"),
                }
            }
        }
        Ok(total)
    }
}
