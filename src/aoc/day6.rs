use core::cmp::PartialEq;
use core::hash::Hash;

use arrayvec::ArrayVec;
use defmt::{debug, error, info};

use nom::bytes::complete::tag;
use nom::combinator::iterator;
use nom::combinator::map_res;
use nom::IResult;
use scapegoat::SgMap;
use scapegoat::SgSet;

use super::utils::direction::Direction;
use super::utils::parse::integer;
use super::utils::parse::list_number;
use super::utils::parse::newline;
use super::utils::parse::non_newline;

type ResultType = u64;

const FULL: &[u8] = include_bytes!("../../input/day6.full");
const SAMPLE: &[u8] = include_bytes!("../../input/day6.sample");

static mut seen: ArrayMap<(isize, isize), u8, 5000> = ArrayMap::new_const();
static mut seen_with_obstacle: ArrayMap<(isize, isize), u8, 8196> = ArrayMap::new_const();

struct ArrayMap<K, V, const CAP: usize> {
    inner: ArrayVec<(K, V), CAP>,
    sort_end: usize,
}
impl<K: PartialEq + Hash + Ord + Copy, V, const CAP: usize> ArrayMap<K, V, CAP> {
    pub const fn new_const() -> Self {
        Self {
            inner: ArrayVec::new_const(),
            sort_end: 0,
        }
    }

    pub fn clear(&mut self) {
        self.inner.clear();
        self.sort_end = 0;
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.inner.push((key, value));
        if self.len() - self.sort_end >= 100 {
            // TODO sort
            debug!("sortage start: {} vs {}", self.len(), self.sort_end);
            self.sort_end = self.len();
            self.inner.sort_unstable_by_key(|&(k, _)| k);
            debug!("end");
        }
    }

    pub fn get_mut(&mut self, key: K) -> Option<&mut (K, V)> {
        if let Ok(idx) = self.inner[..self.sort_end].binary_search_by_key(&key, |&(k, _)| k) {
            self.inner.get_mut(idx)
        } else {
            self.inner[self.sort_end..]
                .iter_mut()
                .find(|(k, _v)| *k == key)
        }
    }
}

impl<'a, K: 'a, V: 'a, const CAP: usize> IntoIterator for &'a ArrayMap<K, V, CAP> {
    type Item = &'a (K, V);
    type IntoIter = core::slice::Iter<'a, (K, V)>;
    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

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
    info!("{} start parsing", label);
    let mut grid = ArrayVec::<&[u8], 200>::new();
    let mut it = iterator(data, grid_line);
    for line in &mut it {
        grid.push(line);
        if line.is_empty() {
            info!("Manual abort");
            break;
        }
    }
    info!("{} read {} gridlines", label, grid.len());
    info!("{} start processing", label);
    unsafe {
        seen.clear();
        analyse(&grid, &mut seen, None);
        let part1_answer = seen.len();
        info!("{} part1 answer = {}", label, part1_answer);

        let mut part2_answer = 0;
        for (a, _d) in &seen {
            seen_with_obstacle.clear();
            if analyse(&grid, &mut seen_with_obstacle, Some((a.0, a.1))) {
                part2_answer += 1;
                info!("{} loop @ {},{}", label, a.0, a.1);
            }
        }
        info!("{} part2 answer = {}", label, part2_answer);
    }
}

fn grid_line(input: &[u8]) -> IResult<&[u8], &[u8]> {
    map_res(nom::sequence::tuple((non_newline, newline)), |(line, _)| {
        Ok::<_, &[u8]>(line)
    })(input)
}

fn analyse<const C: usize, const N: usize>(
    grid: &ArrayVec<&[u8], C>,
    has_seen: &mut ArrayMap<(isize, isize), u8, N>,
    obstacle: Option<(isize, isize)>,
) -> bool {
    let (mut guard_x, mut guard_y, mut direction) = guard_startpos(&grid);
    if matches!(obstacle, Some((x, y)) if x == guard_x && y == guard_y) {
        return false;
    }
    loop {
        debug!(
            "guard: {}, {}, seen {} positions",
            guard_x,
            guard_y,
            has_seen.len()
        );
        let d = match direction {
            Direction::N => 1,
            Direction::E => 2,
            Direction::S => 4,
            Direction::W => 8,
            _ => unreachable!(),
        };
        match has_seen.get_mut((guard_x, guard_y)) {
            Some((_k, v)) => {
                if *v & d != 0 {
                    return true;
                }
                *v |= d;
            }
            None => {
                has_seen.insert((guard_x, guard_y), d);
            }
        };
        let (dx, dy, rotated_direction) = match direction {
            Direction::N => (0, -1, Direction::E),
            Direction::E => (1, 0, Direction::S),
            Direction::S => (0, 1, Direction::W),
            Direction::W => (-1, 0, Direction::N),
            _ => unreachable!(),
        };
        // let decision = match get(grid, guard_x + dx, guard_y + dy) {
        //     _ if obstacle
        //         .map(|(ox, oy)| guard_x + dx == ox && guard_y + dy == oy)
        //         .unwrap_or(false) =>
        //     {
        //         Decision::Turn
        //     }
        //     Some(b'.') | Some(b'^') => Decision::Step,
        //     Some(_) => Decision::Turn,
        //     None => Decision::Exit,
        // };
        // (guard_x, guard_y, direction) = match decision {
        //     Decision::Step => (guard_x + dx, guard_y + dy, direction),
        //     Decision::Turn => (guard_x, guard_y, rotated_direction),
        //     Decision::Exit => {
        //         // Doesn't exist - left grid, so didn't loop
        //         return false;
        //     }
        // }
        (guard_x, guard_y, direction) = match get(grid, guard_x + dx, guard_y + dy) {
            _ if obstacle
                .map(|(ox, oy)| guard_x + dx == ox && guard_y + dy == oy)
                .unwrap_or(false) =>
            {
                (guard_x, guard_y, rotated_direction)
            }
            Some(b'.') | Some(b'^') => (guard_x + dx, guard_y + dy, direction),
            Some(_) => (guard_x, guard_y, rotated_direction),
            None => {
                // Doesn't exist - left grid, so didn't loop
                return false;
            }
        }
    }
}

fn guard_startpos<const C: usize>(grid: &ArrayVec<&[u8], C>) -> (isize, isize, Direction) {
    for (y, r) in grid.iter().enumerate() {
        for (x, c) in r.iter().enumerate() {
            if c == &b'^' {
                return (x as isize, y as isize, Direction::N);
            }
        }
    }
    panic!("Cannot find the guard in the grid");
}

fn get<'a, const C: usize>(grid: &'a ArrayVec<&[u8], C>, x: isize, y: isize) -> Option<&'a u8> {
    if x < 0 || y < 0 {
        None
    } else {
        grid.get(y as usize).and_then(|r| r.get(x as usize))
    }
}
