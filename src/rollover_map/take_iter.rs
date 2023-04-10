use std::{mem, slice};

pub struct TakeIter<'a, T: Default>(slice::IterMut<'a, T>);

impl<'a, T: Default> Drop for TakeIter<'a, T> {
    fn drop(&mut self) {
        for item in &mut self.0 {
            *item = T::default();
        }
    }
}

impl<'a, T: Default> TakeIter<'a, T> {
    pub fn new(slice: &'a mut [T]) -> Self {
        Self(slice.iter_mut())
    }
}

impl<'a, T: Default> Iterator for TakeIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(mem::take)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a, T: Default> ExactSizeIterator for TakeIter<'a, T> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a, T: Default> DoubleEndedIterator for TakeIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().map(mem::take)
    }
}
