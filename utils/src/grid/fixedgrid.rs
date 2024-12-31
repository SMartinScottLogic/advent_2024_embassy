use core::{
    fmt::Debug,
    hash::Hash,
    ops::{Add, AddAssign, Sub},
    range::Step,
};

use crate::point::Point;

use super::Range;

pub struct FixedGrid<T, const C: usize>
where
    [(); C * C]:,
{
    inner: [T; C * C],
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
            inner: core::array::from_fn(|_idx| T::default()),
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
        self.inner[idx] = value;
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
        self.inner.get(idx)
    }
}
trait ToUsize {
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
