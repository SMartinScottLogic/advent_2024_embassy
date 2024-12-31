use core::{
    fmt::Debug,
    hash::Hash,
    ops::{Add, AddAssign, Sub},
    range::Step,
};

use log::{debug, info};

use crate::point::Point;

use super::Range;

pub struct FixedGrid<T, const C: usize>
where
    [(); C * C]:,
{
    inner: [Option<T>; C * C],
}
impl<T: Default, const C: usize> Default for FixedGrid<T, C>
where
    [(); C * C]:,
{
    fn default() -> Self
    where
        [(); C * C]:,
    {
        Self::new()
    }
}
impl<T: Debug, const C: usize> Debug for FixedGrid<T, C>
where
    [(); C * C]:,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
    where
        [(); C * C]:,
    {
        f.debug_struct("FixedGrid")
            .field("width", &C)
            .field("height", &C)
            .finish()
    }
}
impl<T: Default, const C: usize> FixedGrid<T, C>
where
    [(); C * C]:,
{
    pub fn new() -> Self
    where
        [(); C * C]:,
    {
        Self {
            inner: core::array::from_fn(|_idx| None),
        }
    }
}
impl<T: Clone, const C: usize> FixedGrid<T, C>
where
    [(); C * C]:,
{
    pub fn filled_with(val: T) -> Self {
        Self {
            inner: core::array::from_fn(|_idx| Some(val.clone())),
        }
    }
}
impl<T, const C: usize> FixedGrid<T, C>
where
    [(); C * C]:,
{
    pub fn dimensions(&self) -> Range<isize> {
        Range {
            x: 0..=(C - 1) as isize,
            y: 0..=(C - 1) as isize,
        }
    }

    pub fn set<U>(&mut self, position: &Point<U>, value: T)
    where
        U: AddAssign
            + Add<Output = U>
            + Default
            + Hash
            + Ord
            + PartialEq
            + Step
            + Sub<Output = U>
            + Eq
            + Copy
            + ToUsize,
    {
        let idx = position.x().to_usize() + position.y().to_usize() * C;
        self.inner[idx] = Some(value);
    }

    pub fn get<U>(&self, position: &Point<U>) -> Option<&T>
    where
        U: AddAssign
            + Add<Output = U>
            + Default
            + Hash
            + Ord
            + PartialEq
            + Step
            + Sub<Output = U>
            + Eq
            + Copy
            + ToUsize,
    {
        let idx = position.x().to_usize() + position.y().to_usize() * C;
        match self.inner.get(idx) {
            Some(v) => v.as_ref(),
            None => None,
        }
    }

    pub fn get_mut<U>(&mut self, position: &Point<U>) -> Option<&mut T>
    where
        U: AddAssign
            + Add<Output = U>
            + Default
            + Hash
            + Ord
            + PartialEq
            + Step
            + Sub<Output = U>
            + Eq
            + Copy
            + ToUsize,
    {
        let idx = position.x().to_usize() + position.y().to_usize() * C;
        match self.inner.get_mut(idx) {
            Some(v) => v.as_mut(),
            None => None,
        }
    }
}
pub trait ToUsize {
    fn to_usize(&self) -> usize;
}
impl ToUsize for usize {
    fn to_usize(&self) -> usize {
        *self
    }
}
impl ToUsize for isize {
    fn to_usize(&self) -> usize {
        *self as usize
    }
}
impl ToUsize for i32 {
    fn to_usize(&self) -> usize {
        *self as usize
    }
}

impl<T, const C: usize> FixedGrid<T, C>
where
    [(); C * C]:,
{
    pub fn iter(&self) -> FixedGridIter<T, C> {
        FixedGridIter::new(self)
    }
}
pub struct FixedGridIter<'a, T, const C: usize>
where
    [(); C * C]:,
{
    position: Point<isize>,
    data: &'a FixedGrid<T, C>,
}
impl<'a, T, const C: usize> FixedGridIter<'a, T, C>
where
    [(); C * C]:,
{
    fn new(data: &'a FixedGrid<T, C>) -> Self {
        Self {
            data,
            position: Point::new(0, 0),
        }
    }

    fn increment_position(&mut self) {
        let (mut x, mut y) = self.position.ref_mut();

        *x += 1;
        if (*x).to_usize() >= C {
            *x = 0;
            *y += 1;
        }
    }
}
impl<'a, T, const C: usize> Iterator for FixedGridIter<'a, T, C>
where
    [(); C * C]:,
{
    type Item = ((isize, isize), &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.position.y().to_usize() >= C {
                break None;
            }
            let r = self
                .data
                .get(&self.position)
                .map(|val| ((self.position.x(), self.position.y()), val));
            self.increment_position();
            if r.is_some() {
                break r;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::point::Point;

    use super::FixedGrid;

    #[test]
    fn iteration() {
        let mut grid: FixedGrid<_, 4> = FixedGrid::new();
        grid.set(&Point::new(1, 1), 1);
        grid.set(&Point::new(2, 2), 2);
        let mut it = grid.iter();
        assert_eq!(it.next().unwrap(), ((1_isize, 1_isize), &1));
        assert_eq!(it.next().unwrap(), ((2_isize, 2_isize), &2));
        assert!(it.next().is_none());
    }
}
