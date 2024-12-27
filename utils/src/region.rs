use core::hash::Hash;
use core::iter::Step;
use core::ops::{Add, AddAssign, Sub};

use alloc::collections::btree_set::BTreeSet;

use crate::point::Point;

#[derive(Debug, Default)]
pub struct Region<V>
where
    V: Copy + Sub<Output = V> + Add<Output = V> + AddAssign + Ord + Eq + Hash,
{
    elements: BTreeSet<Point<V>>,
}
impl<V> Region<V>
where
    V: Copy + Sub<Output = V> + Add<Output = V> + AddAssign + Ord + Eq + Hash + Default + Step,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn points(&self) -> &BTreeSet<Point<V>> {
        &self.elements
    }

    pub fn area(&self) -> usize {
        self.elements.len()
    }

    pub fn perimeter(&self) -> usize {
        let mut perimeter = 0;
        let mut shared = 0;
        for e in &self.elements {
            for n in e.cardinal() {
                if self.elements.contains(&n) {
                    shared += 1;
                }
            }
            perimeter += 4;
        }

        perimeter - shared
    }

    pub fn num_sides(&self) -> usize {
        self.num_corners()
    }

    pub fn num_corners(&self) -> usize {
        let mut num_sides = 0;
        for e in &self.elements {
            // Outer corners
            if !self.elements.contains(&e.west()) && !self.elements.contains(&e.south()) {
                num_sides += 1;
            }
            if !self.elements.contains(&e.east()) && !self.elements.contains(&e.south()) {
                num_sides += 1;
            }
            if !self.elements.contains(&e.west()) && !self.elements.contains(&e.north()) {
                num_sides += 1;
            }
            if !self.elements.contains(&e.east()) && !self.elements.contains(&e.north()) {
                num_sides += 1;
            }
            // Inner corners
            if self.elements.contains(&e.west())
                && self.elements.contains(&e.south())
                && !self.elements.contains(&e.southwest())
            {
                num_sides += 1;
            }
            if self.elements.contains(&e.east())
                && self.elements.contains(&e.south())
                && !self.elements.contains(&e.southeast())
            {
                num_sides += 1;
            }
            if self.elements.contains(&e.west())
                && self.elements.contains(&e.north())
                && !self.elements.contains(&e.northwest())
            {
                num_sides += 1;
            }
            if self.elements.contains(&e.east())
                && self.elements.contains(&e.north())
                && !self.elements.contains(&e.northeast())
            {
                num_sides += 1;
            }
        }
        num_sides
    }

    pub(super) fn insert(&mut self, point: Point<V>) -> bool {
        self.elements.insert(point)
    }
}
