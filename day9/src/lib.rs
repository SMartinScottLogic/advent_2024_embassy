#![no_std]
#![allow(unused_imports)]
#![feature(generic_const_exprs)]

extern crate core;

use core::num::ParseIntError;

use log::{debug, info};
use scapegoat::SgSet;
use utils::{Solution as _, collections::FixedVec};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    diskmap: FixedVec<ResultType, 20480>,
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
        self.diskmap = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as ResultType)
            .collect();
        Ok(())
    }

    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Result<Self::ResultType, utils::Error> {
        // Implement for problem
        let mut blocks = diskmap(self.diskmap.as_ref());
        let mut s = 0;
        let mut e = blocks.len() - 1;

        loop {
            while blocks[s] != Block::Empty {
                s += 1;
            }
            while blocks[e] == Block::Empty {
                e -= 1;
            }
            if s >= e {
                break;
            }
            blocks[s] = blocks[e];
            blocks[e] = Block::Empty;
        }
        debug!("blocks: {:?}", blocks);

        Ok(checksum(blocks.as_ref()))
    }

    fn answer_part2(&self, _is_full: bool) -> Result<Self::ResultType, utils::Error> {
        // Implement for problem
        let mut handled: SgSet<Block, 10000> = SgSet::new();
        let mut blocks = diskmap(self.diskmap.as_ref());

        let mut e = (blocks.len() - 1) as isize;

        loop {
            while e >= 0
                && (blocks[e as usize] == Block::Empty || handled.contains(&blocks[e as usize]))
            {
                e -= 1;
                debug!("{} {}", e, (e >= 0));
            }
            if e < 0 {
                break;
            }
            handled.insert(blocks[e as usize]);
            let mut num_e = 0;
            while (e - num_e) >= 0 && blocks[e as usize] == blocks[(e - num_e) as usize] {
                num_e += 1;
            }
            debug!("block {:?} {}", blocks[e as usize], num_e);
            let mut s = 0;
            let (s, num_s, can_move) = loop {
                while s < (e - num_e) as isize && blocks[s as usize] != Block::Empty {
                    s += 1;
                }
                if s >= (e - num_e) as isize || blocks[s as usize] != Block::Empty {
                    break (s, 0, false);
                }
                let mut num_s = 1;
                while num_s < num_e
                    && ((s + num_s) as usize) < blocks.len()
                    && blocks[(s + num_s) as usize] == Block::Empty
                {
                    num_s += 1;
                }
                if num_s >= num_e {
                    break (s, num_s, true);
                }
                s += num_s;
            };
            debug!("{} {} {} {} {}", s, num_s, e, num_s, can_move);
            if can_move {
                for i in 0..num_e {
                    blocks[(s + i) as usize] = blocks[(e - num_e + 1 + i) as usize];
                    blocks[(e - num_e + 1 + i) as usize] = Block::Empty;
                }
                // if event_enabled!(Level::DEBUG) {
                //     debug!(
                //         result = blocks.iter().fold(String::new(), |mut a, b| {
                //             match b {
                //                 Block::Empty => a.push('.'),
                //                 Block::FileBlock(id) => a.push_str(&format!("{}", id)),
                //             };
                //             a
                //         })
                //     );
                // }
            }
        }
        debug!("blocks: {:?}", blocks);

        Ok(checksum(blocks.as_ref()))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
enum Block {
    #[default]
    Empty,
    FileBlock(ResultType),
}

fn diskmap(map: &[ResultType]) -> FixedVec<Block, 100000> {
    let mut blocks = FixedVec::new();
    for (id, c) in map.iter().enumerate() {
        let s = if id % 2 == 0 {
            Block::FileBlock((id / 2).try_into().unwrap())
        } else {
            Block::Empty
        };
        debug!("{}: {:?} x {}", id, s, c);
        for _ in 0..*c {
            blocks.push(s);
        }
    }
    debug!("blocks: {:?}", blocks);
    blocks
}

fn checksum(blocks: &[Block]) -> ResultType {
    blocks
        .iter()
        .enumerate()
        .map(|(pos, block)| {
            if let Block::FileBlock(id) = block {
                id * (pos as ResultType)
            } else {
                0
            }
        })
        .sum()
}
