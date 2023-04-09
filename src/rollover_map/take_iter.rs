use std::{mem, slice};

pub struct TakeIter<'a, T: Default> {
    iter: slice::IterMut<'a, T>,
}

impl<'a, T: Default> Drop for TakeIter<'a, T> {
    fn drop(&mut self) {
        for item in &mut self.iter {
            *item = T::default();
        }
    }
}

impl<'a, T: Default> TakeIter<'a, T> {
    pub fn new(slice: &'a mut [T]) -> Self {
        Self {
            iter: slice.iter_mut(),
        }
    }
}

impl<'a, T: Default> Iterator for TakeIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(mem::take)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, T: Default> ExactSizeIterator for TakeIter<'a, T> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, T: Default> DoubleEndedIterator for TakeIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(mem::take)
    }
}
