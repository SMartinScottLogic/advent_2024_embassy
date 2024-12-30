use crate::point::Point;
use crate::region::Region;

use alloc::collections::btree_map::BTreeMap;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use super::Range;
use alloc::borrow::ToOwned;
use core::fmt::{Debug, Display};
use core::hash::Hash;
use core::ops::{Add, AddAssign, Sub};
use core::range::Step;

use log::debug;

#[derive(Debug, Clone)]
pub struct SparseGrid<T, V>
where
    V: Default
        + Sized
        + Copy
        + Sub<Output = V>
        + Add<Output = V>
        + AddAssign
        + Ord
        + Eq
        + PartialEq
        + Hash,
{
    data: BTreeMap<Point<V>, T>,
    range: Range<V>,
}
impl<T, V> SparseGrid<T, V>
where
    T: Default + Display + Clone,
    V: Default
        + Sized
        + Copy
        + Sub<Output = V>
        + Add<Output = V>
        + AddAssign
        + Ord
        + Eq
        + PartialEq
        + Hash
        + Step,
{
    pub fn new() -> Self {
        Self {
            data: BTreeMap::new(),
            range: Range::new(),
        }
    }
}
impl<T, V> Default for SparseGrid<T, V>
where
    T: Default + Display + Clone,
    V: Default
        + Sized
        + Copy
        + Sub<Output = V>
        + Add<Output = V>
        + AddAssign
        + Ord
        + Eq
        + PartialEq
        + Hash
        + Step,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, V> SparseGrid<T, V>
where
    T: Default + Display + Clone,
    V: Default
        + Debug
        + Sized
        + Copy
        + Sub<Output = V>
        + Add<Output = V>
        + AddAssign
        + Ord
        + Eq
        + PartialEq
        + Hash
        + Step,
{
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn get(&self, point: &Point<V>) -> Option<&T> {
        self.data.get(point)
    }

    pub fn set(&mut self, point: &Point<V>, value: T)
    where
        V: Step,
    {
        self.data.insert(point.to_owned(), value);
        let empty = self.range.x.is_empty();
        if empty || self.range.x.start() > &point.x() {
            self.range.x = point.x()..=*self.range.x.end();
        }
        if empty || self.range.x.end() < &point.x() {
            self.range.x = *self.range.x.start()..=point.x();
        }
        let empty = self.range.y.is_empty();
        if empty || self.range.y.start() > &point.y() {
            self.range.y = point.y()..=*self.range.y.end();
        }
        if empty || self.range.y.end() < &point.y() {
            self.range.y = *self.range.y.start()..=point.y();
        }
    }

    pub fn dimensions(&self) -> &Range<V> {
        &self.range
    }

    pub fn min_x(&self) -> &V {
        self.range.x.start()
    }

    pub fn max_x(&self) -> &V {
        self.range.x.end()
    }

    pub fn min_y(&self) -> &V {
        self.range.y.start()
    }

    pub fn max_y(&self) -> &V {
        self.range.y.end()
    }

    pub fn display(&self)
    where
        V: Display + Step,
    {
        self.display_with_mapping(|v| format!("{v}"));
    }
    pub fn display_with_mapping<F>(&self, mapping: F)
    where
        F: Fn(T) -> String,
        V: Display + Step,
    {
        for y in self.range.y.clone() {
            let mut line = String::new();
            line.push_str(&format!("{} ", y));
            for x in self.range.x.clone() {
                let v = match self.get(&Point::new(x, y)) {
                    Some(v) => (*v).to_owned(),
                    None => T::default(),
                };
                let v = mapping(v);
                line.push_str(&v);
            }
            // TODO Work out how to generally output. Temp = log
            log::info!("{line}");
        }
    }

    pub fn contains(&self, point: &Point<V>) -> bool {
        debug!(
            "{:?} {:?} {} {} {} {}",
            point,
            self.range,
            point.x() >= *self.min_x(),
            point.x() <= *self.max_x(),
            point.y() >= *self.min_y(),
            point.y() <= *self.max_y()
        );

        point.x() >= *self.min_x()
            && point.x() <= *self.max_x()
            && point.y() >= *self.min_y()
            && point.y() <= *self.max_y()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Point<V>, &T)> {
        self.data.iter()
    }
}
impl<T, V> SparseGrid<T, V>
where
    T: Default + Display + Clone + PartialEq,
    V: Default
        + Debug
        + Sized
        + Copy
        + Sub<Output = V>
        + Add<Output = V>
        + AddAssign
        + Ord
        + Eq
        + PartialEq
        + Hash
        + Step,
{
    pub fn region_with_same_value(&self, probe: &Point<V>) -> Option<Region<V>> {
        self.get(probe).map(|probe_value| {
            let mut region = Region::new();
            //let mut region = HashSet::new();
            let mut remaining = Vec::new();
            remaining.push(*probe);
            while let Some(cur) = remaining.pop() {
                if !region.insert(cur) {
                    continue;
                }
                for neigh in cur.cardinal() {
                    if let Some(v) = self.get(&neigh) {
                        if v == probe_value {
                            remaining.push(neigh);
                        }
                    }
                }
            }
            region
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid() {
        let mut grid = SparseGrid::new();
        grid.set(&Point::new(1, 1), 1);
        let result = grid.get(&Point::new(1, 1));
        assert_eq!(result, Some(&1i64));
    }

    #[test]
    fn range_1() {
        let mut grid = SparseGrid::new();
        grid.set(&Point::new(1, 1), 1);
        let dim = grid.dimensions();
        assert_eq!(1, *dim.x.start());
        assert_eq!(1, *dim.x.end());
        assert_eq!(1, *dim.x.start());
        assert_eq!(1, *dim.y.end());
    }

    #[test]
    fn iter() {
        let mut grid = SparseGrid::new();
        grid.set(&Point::new(1, 1), 1);
        grid.set(&Point::new(2, 2), 2);
        assert_eq!(2, grid.iter().count());
    }
}
