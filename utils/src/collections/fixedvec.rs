use core::fmt::Debug;

pub struct FixedVec<T, const C: usize> {
    inner: [T; C],
    write_pos: usize,
}
impl<T, const C: usize> Debug for FixedVec<T, C> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("FixedVec")
            .field("capacity", &self.inner.len())
            .field("size", &self.write_pos)
            .finish()
    }
}
impl<T: Default, const C: usize> Default for FixedVec<T, C> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T: Clone, const C: usize> Clone for FixedVec<T, C> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            write_pos: self.write_pos.clone(),
        }
    }
}

impl<T: Default, const C: usize> FixedVec<T, C> {
    pub fn new() -> Self {
        Self {
            inner: core::array::from_fn(|_idx| T::default()),
            write_pos: 0,
        }
    }
}

impl<T: Ord, const C: usize> FixedVec<T, C> {
    pub fn sort(&mut self) {
        self.inner[..self.write_pos].sort();
    }
}

impl<T, const C: usize> FixedVec<T, C> {
    pub fn sort_by_key<K, F>(&mut self, f: F)
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        self.inner.sort_by_key(f);
    }
}

impl<T, const C: usize> FixedVec<T, C> {
    pub fn clear(&mut self) {
        self.write_pos = 0;
    }

    pub fn len(&self) -> usize {
        self.write_pos
    }
    pub fn as_ref(&self) -> &[T] {
        &self.inner[..self.write_pos]
    }
    pub fn as_mut_ref(&mut self) -> &mut [T] {
        &mut self.inner[..self.write_pos]
    }
    pub fn push(&mut self, element: T) {
        assert!(self.write_pos < C);
        self.inner[self.write_pos] = element;
        self.write_pos += 1;
    }
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.write_pos {
            None
        } else {
            Some(&self.inner[index])
        }
    }

    pub fn iter(&self) -> FixedVecIter<T, C> {
        FixedVecIter { pos: 0, data: self }
    }
}

impl<T: Default, const C: usize> FromIterator<T> for FixedVec<T, C> {
    fn from_iter<U>(iter: U) -> Self
    where
        U: IntoIterator<Item = T>,
    {
        let mut c = FixedVec::new();
        for i in iter {
            c.push(i);
        }
        c
    }
}
pub struct FixedVecIter<'a, T, const C: usize> {
    pos: usize,
    data: &'a FixedVec<T, C>,
}
impl<'a, T, const C: usize> Iterator for FixedVecIter<'a, T, C> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.data.write_pos {
            None
        } else {
            self.pos += 1;
            self.data.get(self.pos - 1)
        }
    }
}
