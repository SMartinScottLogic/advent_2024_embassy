use core::cmp::Ordering;

use arrayvec::ArrayVec;
use defmt::{debug, error, info};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{iterator, map_res},
    multi::fold_many1,
    sequence::tuple,
    IResult,
};

use super::utils::parse::{integer, newline};

type ResultType = u64;

const FULL: &[u8] = include_bytes!("../../input/day5.full");
const SAMPLE: &[u8] = include_bytes!("../../input/day5.sample");

pub struct Solution {}
impl super::utils::Solution for Solution {
    fn new() -> impl super::utils::Solution {
        Self {}
    }

    fn run_sample(&mut self) {
        run("sample", SAMPLE)
    }

    fn run_full(&mut self) {
        run("full", FULL)
    }
}

fn run(label: &'static str, data: &[u8]) {
    info!("{} start parsing", label);

    let mut ordering = ArrayVec::<_, 1200>::new();
    let mut updates = ArrayVec::<_, 256>::new();
    let mut it = iterator(data, parse);
    for (row, step) in (&mut it).enumerate() {
        match step {
            Step::Ordering(lhs, rhs) => {
                debug!("{} {}: ordering", label, row);
                ordering.push((lhs, rhs));
            }
            Step::Update(u) => {
                debug!("{} {}: update", label, row);
                updates.push(u);
            }
        }
    }
    match it.finish() {
        Ok((r, _)) => {
            if !r.is_empty() {
                panic!("{} residual size: {}", label, r.len())
            }
        }
        Err(_e) => error!("{} error", label),
    }
    debug!("{} orderings: {}", label, ordering.len());
    debug!("{} updates: {}", label, updates.len());

    info!("{} start processing", label);

    let mut part1_answer = 0;
    for update in updates.iter() {
        let correct = is_correct(update.as_ref(), ordering.as_ref());
        if correct {
            let mid = update.get(update.len() / 2).unwrap();
            part1_answer += mid;
        }
    }
    // Implement for problem
    info!("{} part1 answer = {}", label, part1_answer);

    let mut part2_answer = 0;
    for update in updates.iter() {
        let correct = is_correct(update.as_ref(), ordering.as_ref());
        if !correct {
            let mut fixed = update.clone();
            fix(fixed.as_mut(), ordering.as_ref());
            let mid = fixed.get(update.len() / 2).unwrap();
            part2_answer += mid;
        }
    }
    // Implement for problem
    info!("{} part2 answer = {}", label, part2_answer);
}

fn is_correct(update: &[ResultType], rules: &[(ResultType, ResultType)]) -> bool {
    for (i, page) in update.iter().enumerate() {
        for j in 0..i {
            let probe = update.get(j).unwrap();
            if rules.contains(&(*page, *probe)) {
                return false;
            }
        }
    }
    true
}

fn fix(arr: &mut [ResultType], rules: &[(ResultType, ResultType)]) {
    arr.sort_unstable_by(|a, b| {
        if rules.contains(&(*b, *a)) {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });
}

enum Step {
    Ordering(ResultType, ResultType),
    Update(ArrayVec<ResultType, 32>),
}

fn ordering(input: &[u8]) -> IResult<&[u8], Step> {
    map_res(
        tuple((integer, tag(b"|"), integer, newline)),
        |(lhs, _, rhs, _)| Ok::<_, &[u8]>(Step::Ordering(lhs, rhs)),
    )(input)
}

fn list_number<RT, const C: usize>(input: &[u8]) -> IResult<&[u8], ArrayVec<RT, C>>
where
    RT: core::convert::TryFrom<i32>
        + core::convert::TryFrom<u8>
        + core::ops::MulAssign<RT>
        + core::ops::AddAssign
        + core::default::Default,
    <RT as core::convert::TryFrom<u8>>::Error: core::fmt::Debug,
    <RT as core::convert::TryFrom<i32>>::Error: core::fmt::Debug,
{
    fold_many1(
        tuple((integer::<RT>, tag(b","))),
        ArrayVec::new,
        |mut acc: ArrayVec<RT, C>, item| {
            acc.push(item.0);
            acc
        },
    )(input)
}

fn update(input: &[u8]) -> IResult<&[u8], Step> {
    map_res(tuple((list_number, integer, newline)), |(mut u, v, _)| {
        u.push(v);
        Ok::<_, &[u8]>(Step::Update(u))
    })(input)
}
fn parse(input: &[u8]) -> IResult<&[u8], Step> {
    alt((ordering, update))(input)
}
