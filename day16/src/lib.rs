#![no_std]
#![allow(unused_imports)]
#![feature(generic_const_exprs)]

extern crate core;

use core::num::ParseIntError;

use log::{debug, info};
use scapegoat::SgSet;
use utils::grid::FixedGrid;
use utils::point::{Direction, Point};
use utils::{Solution as _, collections::FixedVec};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    grid: FixedGrid<char, 150>,
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
    fn update_from_line(&mut self, y: usize, line: &str) -> Result<(), Self::ParseError> {
        for (x, c) in line.chars().enumerate() {
            self.grid.set(&Point::new(x, y), c);
        }
        Ok(())
    }

    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Result<Self::ResultType, utils::Error> {
        let mut start = (Direction::E, -1, -1);
        let mut end = (-1, -1);

        for ((x, y), c) in self.grid.iter() {
            match *c {
                'S' => start = (Direction::E, x, y),
                'E' => end = (x, y),
                _ => {}
            }
        }

        let answer = if let Some((_route, cost)) = pathfinding::directed::astar::astar(
            &start,
            |p| Self::successors(&self.grid, p.0, p.1, p.2),
            |p| Self::heuristic(p.0, p.1, p.2, &end),
            |p| Self::success(p.0, p.1, p.2, &end),
        ) {
            cost
        } else {
            0
        };
        // Implement for problem
        Ok(answer)
    }

    fn answer_part2(&self, _is_full: bool) -> Result<Self::ResultType, utils::Error> {
        let mut start = (Direction::E, -1, -1);
        let mut end = (-1, -1);
        for ((x, y), c) in self.grid.iter() {
            match *c {
                'S' => start = (Direction::E, x, y),
                'E' => end = (x, y),
                _ => {}
            }
        }

        let answer = if let Some((routes, cost)) = pathfinding::directed::astar::astar_bag_collect(
            &start,
            |p| Self::successors(&self.grid, p.0, p.1, p.2),
            |p| Self::heuristic(p.0, p.1, p.2, &end),
            |p| Self::success(p.0, p.1, p.2, &end),
        ) {
            debug!("routes: {:?}, cost: {}", routes, cost);
            let nodes = routes
                .iter()
                .flat_map(|v| v.iter().map(|&(_facing, x, y)| (x, y)))
                .collect::<SgSet<_, 1500>>();
            nodes.len() as ResultType
        } else {
            0
        };
        // Implement for problem
        Ok(answer)
    }
}

impl Solution {
    fn success(_facing: Direction, x: isize, y: isize, end: &(isize, isize)) -> bool {
        end.0 == x && end.1 == y
    }
    fn heuristic(_facing: Direction, x: isize, y: isize, end: &(isize, isize)) -> ResultType {
        (end.0.abs_diff(x) + end.1.abs_diff(y)) as ResultType
    }
    fn successors(
        grid: &FixedGrid<char, 150>,
        facing: Direction,
        x: isize,
        y: isize,
    ) -> impl Iterator<Item = ((Direction, isize, isize), ResultType)> {
        match facing {
            Direction::N => [
                ((Direction::E, x, y), 1000),
                ((Direction::W, x, y), 1000),
                ((Direction::N, x, y - 1), 1),
            ],
            Direction::S => [
                ((Direction::E, x, y), 1000),
                ((Direction::W, x, y), 1000),
                ((Direction::S, x, y + 1), 1),
            ],
            Direction::E => [
                ((Direction::N, x, y), 1000),
                ((Direction::S, x, y), 1000),
                ((Direction::E, x + 1, y), 1),
            ],
            Direction::W => [
                ((Direction::N, x, y), 1000),
                ((Direction::S, x, y), 1000),
                ((Direction::W, x - 1, y), 1),
            ],
            _ => panic!(),
        }
        .into_iter()
        .filter(|&((_facing, x, y), _c)| *grid.get(&Point::new(x, y)).unwrap() != '#')
        .collect::<FixedVec<_, 3>>()
        .into_iter()
    }
}
