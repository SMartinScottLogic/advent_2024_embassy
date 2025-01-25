use defmt::info;

use crate::aoc::utils::FixedVec;

type ResultType = u64;

const FULL: &[u8] = include_bytes!("../../input/day9.full");
const SAMPLE: &[u8] = include_bytes!("../../input/day9.sample");

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
    info!("{} start processing", label);
    let step1_answer = analyse_part1(data);
    info!("{} part1 answer: {}", label, step1_answer);
    let step2_answer = analyse_part2(data);
    info!("{} part2 answer: {}", label, step2_answer);
}

fn analyse_part1(data: &[u8]) -> ResultType {
    let mut blocks = diskmap(data);
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

    checksum(blocks.as_ref())
}

fn analyse_part2(_data: &[u8]) -> ResultType {
    // TODO
    0
}

fn checksum(blocks: &[Block]) -> ResultType {
    blocks
        .iter()
        .enumerate()
        .map(|(pos, block)| {
            if let Block::FileBlock(id) = block {
                (*id as ResultType) * (pos as ResultType)
            } else {
                0
            }
        })
        .sum()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
enum Block {
    #[default]
    Empty,
    FileBlock(usize),
}

fn diskmap(map: &[u8]) -> FixedVec<Block, 200000> {
    let mut blocks = FixedVec::new();
    for (id, c) in map.iter().enumerate() {
        let c = *c - b'0';
        let s = if id % 2 == 0 {
            Block::FileBlock(id / 2)
        } else {
            Block::Empty
        };
        for _ in 0..c {
            blocks.push(s);
        }
    }
    blocks
}
