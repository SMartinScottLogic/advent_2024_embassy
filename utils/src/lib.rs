#![no_std]
#![feature(step_trait)]
#![feature(new_range_api)]
#![feature(generic_const_exprs)]

extern crate alloc;
extern crate core;

pub mod collections;
pub mod grid;
pub mod point;
mod region;
mod solution;

pub use solution::{Error, Solution};
