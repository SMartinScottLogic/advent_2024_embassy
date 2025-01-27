use arrayvec::ArrayVec;
use defmt::{debug, error, info};

use nom::bytes::complete::tag;
use nom::combinator::map_res;
use nom::IResult;

use super::utils::parse::integer;
use super::utils::parse::list_number;
use super::utils::parse::newline;

type ResultType = u64;

const FULL: &[u8] = include_bytes!("../../input/day7.full");
const SAMPLE: &[u8] = include_bytes!("../../input/day7.sample");

pub struct Solution {}
impl super::utils::Solution for Solution {
    fn new() -> impl super::utils::Solution {
        Self {}
    }

    fn run_sample(&self) {
        run("sample", SAMPLE)
    }

    fn run_full(&self) {
        run("full", FULL)
    }
}

fn run(label: &'static str, data: &[u8]) {
    info!("{} start parsing", label);
    let mut data = data;
    let mut row = 0;
    let mut total1 = 0;
    let mut total2 = 0;
    loop {
        if let Ok((r, (answer, values))) = parse(data) {
            info!("{}: {} -> {} values", row, answer, values.len());
            if can_be_true(&answer, values.as_ref(), false) {
                total1 += answer;
            }
            if can_be_true(&answer, values.as_ref(), true) {
                total2 += answer;
            }
            data = r;
        } else {
            error!("{}: Error ({})", row, data[0]);
            break;
        }
        if data.is_empty() {
            break;
        }
        row += 1;
    }
    info!("{} part1 answer: {}", label, total1);
    info!("{} part2 answer: {}", label, total2);
}

fn parse(input: &[u8]) -> IResult<&[u8], (ResultType, ArrayVec<ResultType, 50>)> {
    map_res(
        nom::sequence::tuple((integer, tag(":"), list_number, newline)),
        |(answer, _, params, _)| Ok::<_, &[u8]>((answer, params)),
    )(input)
}

fn can_be_true(answer: &ResultType, values: &[ResultType], is_part2: bool) -> bool {
    let mut operators = values
        .iter()
        .map(|_| '*')
        .skip(1)
        .collect::<ArrayVec<_, 50>>();
    let operators = operators.as_mut();
    test_all_up_to(operators.len() - 1, answer, values, operators, is_part2)
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
