use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};

pub trait Clear {
    fn clear(&mut self);
}

impl<K, V, S> Clear for HashMap<K, V, S> {
    fn clear(&mut self) {
        self.clear()
    }
}

impl<K, V> Clear for BTreeMap<K, V> {
    fn clear(&mut self) {
        self.clear()
    }
}

impl<T> Clear for Vec<T> {
    fn clear(&mut self) {
        self.clear()
    }
}

impl<T, S> Clear for HashSet<T, S> {
    fn clear(&mut self) {
        self.clear()
    }
}

impl<T> Clear for BTreeSet<T> {
    fn clear(&mut self) {
        self.clear()
    }
}

impl<T> Clear for VecDeque<T> {
    fn clear(&mut self) {
        self.clear()
    }
}
