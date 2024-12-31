mod fixedgrid;
mod sparsegrid;

use core::ops::RangeInclusive;
use core::range::Step;

pub use fixedgrid::FixedGrid;
pub use sparsegrid::SparseGrid;

#[derive(Debug, Clone)]
pub struct Range<T> {
    pub x: RangeInclusive<T>,
    pub y: RangeInclusive<T>,
}
impl<T> Range<T>
where
    T: Default + Step,
{
    fn new() -> Self {
        Self {
            x: T::forward(T::default(), 1)..=T::default(),
            y: T::forward(T::default(), 1)..=T::default(),
        }
    }
}
