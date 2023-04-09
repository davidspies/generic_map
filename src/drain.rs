use std::{
    collections::{hash_map, hash_set, vec_deque, HashMap, HashSet, VecDeque},
    vec,
};

pub trait Drain {
    type Output<'a>
    where
        Self: 'a;

    fn drain(&mut self) -> Self::Output<'_>;
}

impl<K, V, S> Drain for HashMap<K, V, S> {
    type Output<'a> = hash_map::Drain<'a, K, V>
    where
        Self: 'a;

    fn drain(&mut self) -> Self::Output<'_> {
        self.drain()
    }
}

impl<T> Drain for Vec<T> {
    type Output<'a> = vec::Drain<'a, T>
    where
        Self: 'a;

    fn drain(&mut self) -> Self::Output<'_> {
        self.drain(..)
    }
}

impl<T, S> Drain for HashSet<T, S> {
    type Output<'a> = hash_set::Drain<'a, T>
    where
        Self: 'a;

    fn drain(&mut self) -> Self::Output<'_> {
        self.drain()
    }
}

impl<T> Drain for VecDeque<T> {
    type Output<'a> = vec_deque::Drain<'a, T>
    where
        Self: 'a;

    fn drain(&mut self) -> Self::Output<'_> {
        self.drain(..)
    }
}
