use arrayvec::ArrayVec;
use static_cell::StaticCell;

use nonmax::NonMaxU16;

use crate::{debug, info};

type ResultType = u64;

const NUM_BLOCKS: usize = 95000;

const FULL: &[u8] = include_bytes!("../../input/day9.full");
const SAMPLE: &[u8] = include_bytes!("../../input/day9.sample");
static BLOCKS: StaticCell<ArrayVec<Block, NUM_BLOCKS>> = StaticCell::new();

pub struct Solution {
    blocks: &'static mut ArrayVec<Block, NUM_BLOCKS>,
}
impl super::utils::Solution for Solution {
    fn new() -> impl super::utils::Solution {
        let blocks = BLOCKS.init_with(|| ArrayVec::new());
        Self { blocks }
    }

    fn run_sample(&mut self) {
        run("sample", SAMPLE, self.blocks)
    }

    fn run_full(&mut self) {
        run("full", FULL, self.blocks)
    }
}

fn run(label: &'static str, data: &[u8], blocks: &mut ArrayVec<Block, NUM_BLOCKS>) {
    info!("{} start processing", label);
    let step1_answer = analyse_part1(data, blocks);
    info!("{} blocks count: {}", label, blocks.len());
    info!("{} part1 answer: {}", label, step1_answer);
    let step2_answer = analyse_part2(data, blocks);
    info!("{} part2 answer: {}", label, step2_answer);
}

fn analyse_part1(data: &[u8], blocks: &mut ArrayVec<Block, NUM_BLOCKS>) -> ResultType {
    diskmap(data, blocks);
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

fn analyse_part2(data: &[u8], blocks: &mut ArrayVec<Block, NUM_BLOCKS>) -> ResultType {
    diskmap(data, blocks);

    let mut e = (blocks.len() - 1) as isize;
    let mut seen = u16::MAX;

    loop {
        while e >= 0
            && (
                match blocks[e as usize] {
                    Block::FileBlock(b) if b.get() < seen => false,
                    _ => true,
                }
                // || handled.contains(&blocks[e as usize])
            )
        {
            e -= 1;
            debug!("e: {}, end: {}", e, (e >= 0));
        }
        if e < 0 {
            break;
        }
        //handled.insert(blocks[e as usize]);
        match blocks[e as usize] {
            Block::FileBlock(b) => {
                seen = b.get();
            }
            _ => panic!(),
        };
        let mut num_e = 0;
        while (e - num_e) >= 0 && blocks[e as usize] == blocks[(e - num_e) as usize] {
            num_e += 1;
        }
        debug!("block: {:?}, num_e: {}", blocks[e as usize], num_e);
        let mut s = 0;
        let (s, num_s, can_move) = loop {
            while s < (e - num_e) && blocks[s as usize] != Block::Empty {
                s += 1;
            }
            if s >= (e - num_e) || blocks[s as usize] != Block::Empty {
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
        debug!(
            "s: {}, num_s: {}, e: {}, can_move: {}",
            s, num_s, e, can_move
        );
        if can_move {
            for i in 0..num_e {
                blocks[(s + i) as usize] = blocks[(e - num_e + 1 + i) as usize];
                blocks[(e - num_e + 1 + i) as usize] = Block::Empty;
            }
        }
    }
    debug!("blocks: {:?}", blocks);

    checksum(blocks.as_ref())
}

fn checksum(blocks: &[Block]) -> ResultType {
    blocks
        .iter()
        .enumerate()
        .map(|(pos, block)| {
            if let Block::FileBlock(id) = block {
                (id.get() as ResultType) * (pos as ResultType)
            } else {
                0
            }
        })
        .sum()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
enum Block {
    #[default]
    Empty,
    FileBlock(NonMaxU16),
}

fn diskmap(map: &[u8], blocks: &mut ArrayVec<Block, NUM_BLOCKS>) {
    blocks.clear();
    for (id, c) in map.iter().enumerate() {
        let c = *c - b'0';
        let s = if id % 2 == 0 {
            let id = NonMaxU16::new((id / 2) as u16).unwrap();
            Block::FileBlock(id)
        } else {
            Block::Empty
        };
        for _ in 0..c {
            blocks.push(s);
        }
    }
}
