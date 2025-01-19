use defmt::{debug, error, info};

use nom::bytes::complete::tag;
use nom::bytes::complete::take_while;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::multi::fold_many1;
use nom::IResult;

use crate::aoc::utils::FixedVec;

pub type ResultType = u64;

const FULL: &'static [u8] = include_bytes!("../../../input/day7.full");
const SAMPLE: &'static [u8] = include_bytes!("../../../input/day7.sample");

pub fn run_sample() {
    run(SAMPLE)
}

pub fn run_full() {
    run(FULL)
}

fn run(data: &[u8]) {
    let mut data = data;
    let mut row = 0;
    let mut total1 = 0;
    let mut total2 = 0;
    loop {
        if let Ok((r, (answer, values))) = parse(data) {
            debug!("{}: {} -> {} values", row, answer, values.len());
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
    info!("Part1 answer: {}", total1);
    info!("Part2 answer: {}", total2);
}

fn number(input: &[u8]) -> IResult<&[u8], ResultType> {
    map_res(digit1, |digits: &[u8]| {
        Ok::<ResultType, &[u8]>(digits.iter().fold(0 as ResultType, |mut acc, v| {
            acc *= 10;
            acc += (v - b'0') as ResultType;
            acc
        }))
    })(input)
}
fn list_number(input: &[u8]) -> IResult<&[u8], FixedVec<ResultType, 50>> {
    fold_many1(
        nom::sequence::tuple((tag(" "), number)),
        FixedVec::new,
        |mut acc: FixedVec<ResultType, 50>, item| {
            acc.push(item.1);
            acc
        },
    )(input)
}
fn newline(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while(|c| c == b'\n' || c == b'\r')(input)
}

fn parse(input: &[u8]) -> IResult<&[u8], (ResultType, FixedVec<ResultType, 50>)> {
    map_res(
        nom::sequence::tuple((number, tag(":"), list_number, newline)),
        |(answer, _, params, _)| Ok::<_, &[u8]>((answer, params)),
    )(input)
}

fn can_be_true(answer: &ResultType, values: &[ResultType], is_part2: bool) -> bool {
    let mut operators = values
        .iter()
        .map(|_| '*')
        .skip(1)
        .collect::<FixedVec<_, 50>>();
    let mut operators = operators.as_mut_ref();
    test_all_up_to(
        operators.len() - 1,
        answer,
        values,
        &mut operators,
        is_part2,
    )
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
