use super::Solution;
use crate::aoc::utils::Solution as _;

pub struct AocTask {}

impl AocTask {
    pub fn new() -> Self {
        Self {}
    }
    pub async fn run(&mut self) {
        let mut solution = Solution::new();
        solution.run_sample();
        solution.run_full();
    }
}
