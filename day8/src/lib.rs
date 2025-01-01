#![no_std]
#![allow(unused_imports)]
#![feature(generic_const_exprs)]

extern crate core;

use core::num::ParseIntError;

use log::{debug, info};
use scapegoat::SgSet;
use utils::{Solution as _, grid::FixedGrid, point::Point};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    grid: FixedGrid<char, 50>,
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
        let mut antinodes: SgSet<Point<isize>, 1024> = SgSet::new();
        for (p1, c1) in self.grid.iter() {
            for (p2, c2) in self.grid.iter() {
                let p1 = Point::new(p1.0, p1.1);
                let p2 = Point::new(p2.0, p2.1);
                if p1 == p2 || c1 != c2 {
                    continue;
                }
                if *c1 == '.' {
                    continue;
                }
                debug!("{:?}, {:?} ; {} {}", p1, p2, c1, c2);

                let dx = p1.x() - p2.x();
                let dy = p1.y() - p2.y();
                // antinode 1
                {
                    let x = p1.x() + dx;
                    let y = p1.y() + dy;
                    let p = Point::new(x, y);
                    if p != p1 && p != p2 && self.grid.contains(&p) {
                        debug!("1: {:?}, {:?}, {:?} {}", p, p1, p2, self.grid.contains(&p));
                        antinodes.insert(p);
                    }
                }
                // antinode 2
                {
                    let x = p2.x() + dx;
                    let y = p2.y() + dy;
                    let p = Point::new(x, y);
                    if p != p1 && p != p2 && self.grid.contains(&p) {
                        debug!("2: {:?}, {:?}, {:?} {}", p, p1, p2, self.grid.contains(&p));
                        antinodes.insert(p);
                    }
                }
                // antinode 3
                {
                    let x = p1.x() - dx;
                    let y = p1.y() - dy;
                    let p = Point::new(x, y);
                    if p != p1 && p != p2 && self.grid.contains(&p) {
                        debug!("3: {:?}, {:?}, {:?} {}", p, p1, p2, self.grid.contains(&p));
                        antinodes.insert(p);
                    }
                }
                // antinode 4
                {
                    let x = p2.x() - dx;
                    let y = p2.y() - dy;
                    let p = Point::new(x, y);
                    if p != p1 && p != p2 && self.grid.contains(&p) {
                        debug!("4: {:?}, {:?}, {:?} {}", p, p1, p2, self.grid.contains(&p));
                        antinodes.insert(p);
                    }
                }
            }
        }
        debug!("antinodes: {:?}", antinodes);
        Ok(antinodes.len() as ResultType)
    }

    fn answer_part2(&self, _is_full: bool) -> Result<Self::ResultType, utils::Error> {
        let mut antinodes: SgSet<Point<_>, 1024> = SgSet::new();
        for (p1, c1) in self.grid.iter() {
            for (p2, c2) in self.grid.iter() {
                let p1 = Point::new(p1.0, p1.1);
                let p2 = Point::new(p2.0, p2.1);
                if p1 == p2 || c1 != c2 {
                    continue;
                }
                if *c1 == '.' {
                    continue;
                }
                debug!("{:?} {:?} {} {}", p1, p2, c1, c2);

                let dx = p1.x() - p2.x();
                let dy = p1.y() - p2.y();
                let mut x = p1.x();
                let mut y = p1.y();

                loop {
                    let p = Point::new(x, y);
                    if !self.grid.contains(&p) {
                        break;
                    }
                    antinodes.insert(p);
                    x -= dx;
                    y -= dy;
                }

                let mut x = p1.x();
                let mut y = p1.y();

                loop {
                    let p = Point::new(x, y);
                    if !self.grid.contains(&p) {
                        break;
                    }
                    antinodes.insert(p);
                    x += dx;
                    y += dy;
                }
            }
        }
        // Implement for problem
        Ok(antinodes.len() as ResultType)
    }
}
