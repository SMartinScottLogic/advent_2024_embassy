#![no_std]
#![allow(unused_imports)]
#![feature(generic_const_exprs)]

extern crate core;

use core::num::ParseIntError;

use log::{debug, info};
use utils::{Solution as _, collections::FixedVec, point::Point};

use nalgebra::{matrix, vector};

pub type ResultType = u64;

type Button = Point<ResultType>;
type Prize = Point<ResultType>;

#[derive(Debug, Default)]
pub struct Solution {
    machines: FixedVec<(Button, Button, Prize), 5>,
    button_a: Option<Button>,
    button_b: Option<Button>,
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
        if line.is_empty() {
            return Ok(());
        }
        let (lhs, rhs) = line.split_once(": ").unwrap();
        debug!("line = {}, lhs = {}, rhs = {}", line, lhs, rhs);
        if lhs == "Button A" {
            if self.button_a.is_some() {
                panic!("second Button A");
            }
            let c = button_regex.captures(rhs).unwrap();
            let x = c.name("x").unwrap().as_str().parse().unwrap();
            let y = c.name("y").unwrap().as_str().parse().unwrap();
            self.button_a = Some(Point::new(x, y));
        } else if lhs == "Button B" {
            if self.button_b.is_some() {
                panic!("second Button B");
            }
            let c = button_regex.captures(rhs).unwrap();
            let x = c.name("x").unwrap().as_str().parse().unwrap();
            let y = c.name("y").unwrap().as_str().parse().unwrap();
            self.button_b = Some(Point::new(x, y));
        } else if lhs == "Prize" {
            let c = prize_regex.captures(rhs).unwrap();
            let x = c.name("x").unwrap().as_str().parse().unwrap();
            let y = c.name("y").unwrap().as_str().parse().unwrap();
            let prize = Point::new(x, y);
            self.machines
                .push((self.button_a.unwrap(), self.button_b.unwrap(), prize));
            self.button_a = None;
            self.button_b = None;
        } else {
            panic!("unexpected line: {}", line);
        }
        Ok(())
    }

    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Result<Self::ResultType, utils::Error> {
        let mut total = 0;
        for machine in self.machines.iter() {
            let mc = min_cost_part1(100, 100, machine);
            debug!("machine = {:?}, mc={:?}", machine, mc);
            if let Some(cost) = mc {
                total += cost;
            }
        }
        // Implement for problem
        Ok(total)
    }

    fn answer_part2(&self, _is_full: bool) -> Result<Self::ResultType, utils::Error> {
        let mut total = 0;
        for (a, b, prize) in self.machines.iter() {
            let prize = Point::new(10000000000000 + prize.x(), 10000000000000 + prize.y());
            let mc = min_cost_part2(&(*a, *b, prize));
            debug!("machine = {:?}, mc = {:?}", (a, b, prize), mc);
            if let Some(cost) = mc {
                total += cost;
            }
        }
        // Implement for problem
        Ok(total)
    }
}

fn min_cost_part1(
    a_left: ResultType,
    b_left: ResultType,
    (button_a, button_b, prize): &(Button, Button, Prize),
) -> Option<ResultType> {
    let location = Point::new(0, 0);
    debug!(
        "aleft = {:?}, b_left = {:?}, location = {:?}",
        a_left, b_left, location
    );
    let a_cost = 3;
    let b_cost = 1;

    for a_presses in 0..=a_left {
        let a_move = location + *button_a * a_presses;
        if a_move.x() > prize.x() && a_move.y() > prize.y() {
            continue;
        }
        let remaining = *prize - a_move;
        if remaining.x() % button_b.x() == 0 && remaining.y() % button_b.y() == 0 {
            let x_presses = remaining.x() / button_b.x();
            let y_presses = remaining.y() / button_b.y();
            if x_presses == y_presses {
                debug!(
                    "a_presses: {:?}, b_presses: {:?}, fast",
                    a_presses, x_presses
                );
                return Some(a_presses * a_cost + x_presses * b_cost);
            }
        }
    }
    None
}

fn min_cost_part2((button_a, button_b, prize): &(Button, Button, Prize)) -> Option<ResultType> {
    // nalgebra
    let m =
        matrix![button_a.x() as f64, button_b.x() as f64; button_a.y() as f64, button_b.y() as f64];
    match m.try_inverse() {
        Some(inv) => {
            let r = inv * vector![prize.x() as f64, prize.y() as f64];
            debug!("r = {:?}", r);
            if r.iter().all(|f| (f - f.round()).abs() < 1e-3) {
                let r = r.transpose() * vector![3.0, 1.0];
                debug!("r = {:?}", r);
                Some(r.magnitude().round() as u64)
            } else {
                None
            }
        }
        None => None,
    }
    // Z3
    // let ctx = Context::new(&Config::default());
    // let solver = Solver::new(&ctx);

    // let a_presses = ast::Int::new_const(&ctx, "a_presses");
    // let b_presses = ast::Int::new_const(&ctx, "b_presses");

    // let prize_x = ast::Int::from_u64(&ctx, prize.x());
    // let prize_y = ast::Int::from_u64(&ctx, prize.y());
    // // X
    // let button_a_x = ast::Int::from_u64(&ctx, button_a.x());
    // let button_b_x = ast::Int::from_u64(&ctx, button_b.x());
    // let rx = &a_presses * &button_a_x + &b_presses * &button_b_x;
    // solver.assert(&rx._eq(&prize_x));

    // // Y
    // let button_a_y = ast::Int::from_u64(&ctx, button_a.y());
    // let button_b_y = ast::Int::from_u64(&ctx, button_b.y());
    // let ry = &a_presses * &button_a_y + &b_presses * &button_b_y;
    // solver.assert(&ry._eq(&prize_y));

    // match solver.check() {
    //     SatResult::Sat => {
    //         let model = solver.get_model().unwrap();
    //         debug!(model = debug(&model));
    //         let a = model
    //             .get_const_interp(&a_presses)
    //             .unwrap()
    //             .as_i64()
    //             .unwrap();
    //         let b = model
    //             .get_const_interp(&b_presses)
    //             .unwrap()
    //             .as_i64()
    //             .unwrap();
    //         Some((a * 3 + b) as ResultType)
    //     }
    //     _ => None,
    // }
}
