use std::collections::{btree_map, BTreeMap};
use std::mem;

use crate::{Entry, GenericMap, OccupiedEntry, VacantEntry};

impl<K: Ord, V> GenericMap<K, V> for BTreeMap<K, V> {
    type Iter<'a> = btree_map::Iter<'a, K, V>
    where
        K: 'a,
        V: 'a;

    type IterMut<'a> = btree_map::IterMut<'a, K, V>
    where
        K: 'a,
        V: 'a;

    type DrainIter<'a> = btree_map::IntoIter<K, V>
    where
        Self: 'a;

    type VacEntry<'a> = btree_map::VacantEntry<'a, K, V>
    where
        Self: 'a;

    type OccupEntry<'a> = btree_map::OccupiedEntry<'a, K, V>
    where
        Self: 'a;

    fn new() -> Self {
        Self::new()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn contains_key(&self, key: &K) -> bool {
        self.contains_key(key)
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.get(key)
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.get_mut(key)
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.insert(key, value)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.remove(key)
    }

    fn drain(&mut self) -> Self::DrainIter<'_> {
        mem::take(self).into_iter()
    }

    fn entry(&mut self, key: K) -> Entry<Self::VacEntry<'_>, Self::OccupEntry<'_>> {
        match self.entry(key) {
            btree_map::Entry::Vacant(v) => Entry::Vacant(v),
            btree_map::Entry::Occupied(o) => Entry::Occupied(o),
        }
    }

    fn iter(&self) -> Self::Iter<'_> {
        self.iter()
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.iter_mut()
    }
}

impl<'a, K: Ord, V> VacantEntry<'a, K, V> for btree_map::VacantEntry<'a, K, V> {
    fn key(&self) -> &K {
        self.key()
    }

    fn insert(self, value: V) -> &'a mut V {
        self.insert(value)
    }
}

impl<'a, K: Ord, V> OccupiedEntry<'a, K, V> for btree_map::OccupiedEntry<'a, K, V> {
    fn key(&self) -> &K {
        self.key()
    }

    fn insert(&mut self, value: V) -> V {
        self.insert(value)
    }

    fn remove(self) -> V {
        self.remove()
    }

    fn get(&self) -> &V {
        self.get()
    }

    fn get_mut(&mut self) -> &mut V {
        self.get_mut()
    }

    fn into_mut(self) -> &'a mut V {
        self.into_mut()
    }
}
