#![no_std]
#![allow(unused_imports)]
#![feature(generic_const_exprs)]

extern crate core;

use core::num::ParseIntError;

use log::{debug, info};
use utils::{Solution as _, collections::FixedVec};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    equations: FixedVec<(ResultType, FixedVec<ResultType, 16>), 1024>,
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
        let (result, rhs) = line.split_once(':').unwrap();
        let result = result.parse()?;
        let rhs = rhs.split_whitespace().map(|v| v.parse().unwrap()).collect();
        self.equations.push((result, rhs));
        Ok(())
    }

    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Result<Self::ResultType, utils::Error> {
        // Implement for problem
        let mut total = 0;
        for (answer, values) in self.equations.iter() {
            if can_be_true(answer, values.as_ref(), false) {
                total += answer;
            }
        }
        Ok(total)
    }

    fn answer_part2(&self, _is_full: bool) -> Result<Self::ResultType, utils::Error> {
        // Implement for problem
        let mut total = 0;
        for (answer, values) in self.equations.iter() {
            if can_be_true(answer, values.as_ref(), true) {
                total += answer;
            }
        }
        Ok(total)
    }
}

fn can_be_true(answer: &ResultType, values: &[ResultType], is_part2: bool) -> bool {
    let mut operators = values
        .iter()
        .map(|_| '*')
        .skip(1)
        .collect::<FixedVec<_, 16>>();
    let mut operators = operators.as_mut_ref();
    test_all_up_to(
        operators.len() - 1,
        answer,
        values,
        &mut operators,
        is_part2,
    )
}

fn evaluate(values: &[ResultType], operators: &[char], is_part2: bool) -> ResultType {
    let mut answer = values[0];
    for p in 0..operators.len() {
        let operator = operators[p];
        let rhs = values[p + 1];
        answer = match operator {
            '*' => answer * rhs,
            '+' => answer + rhs,
            '|' if is_part2 => concatenate(answer, rhs),
            _ => panic!(),
        }
    }
    //info!(?answer);
    answer
}

fn test_all_up_to(
    n: usize,
    answer: &ResultType,
    values: &[ResultType],
    operators: &mut [char],
    is_part2: bool,
) -> bool {
    operators[n] = '*';
    if n > 0 {
        if test_all_up_to(n - 1, answer, values, operators, is_part2) {
            return true;
        }
    } else if evaluate(values, operators, is_part2) == *answer {
        return true;
    }
    operators[n] = '+';
    if n > 0 {
        if test_all_up_to(n - 1, answer, values, operators, is_part2) {
            return true;
        }
    } else if evaluate(values, operators, is_part2) == *answer {
        return true;
    }
    if is_part2 {
        operators[n] = '|';
        if n > 0 {
            if test_all_up_to(n - 1, answer, values, operators, is_part2) {
                return true;
            }
        } else if evaluate(values, operators, is_part2) == *answer {
            return true;
        }
    }
    false
}

fn concatenate(lhs: ResultType, rhs: ResultType) -> ResultType {
    let mut scale = 10;
    loop {
        if rhs < scale {
            break;
        }
        scale *= 10;
    }
    rhs + scale * lhs
}
