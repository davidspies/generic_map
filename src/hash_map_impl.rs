use std::collections::{hash_map, HashMap};
use std::hash::{BuildHasher, Hash};

use crate::{Entry, GenericMap, OccupiedEntry, VacantEntry};

impl<K: Eq + Hash, V, S: BuildHasher + Default> GenericMap for HashMap<K, V, S> {
    type K = K;
    type V = V;
    type Iter<'a> = hash_map::Iter<'a, K, V>
    where
        K: 'a,
        V: 'a,
        S: 'a;

    type IterMut<'a> = hash_map::IterMut<'a, K, V>
    where
        K: 'a,
        V: 'a,
        S: 'a;

    type DrainIter<'a> = hash_map::Drain<'a, K, V>
    where
        Self: 'a;

    type VacEntry<'a> = hash_map::VacantEntry<'a, K, V>
    where
        Self: 'a;

    type OccupEntry<'a> = hash_map::OccupiedEntry<'a, K, V>
    where
        Self: 'a;

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
        self.drain()
    }

    fn entry(&mut self, key: K) -> Entry<Self::VacEntry<'_>, Self::OccupEntry<'_>> {
        match self.entry(key) {
            hash_map::Entry::Vacant(v) => Entry::Vacant(v),
            hash_map::Entry::Occupied(o) => Entry::Occupied(o),
        }
    }

    fn iter(&self) -> Self::Iter<'_> {
        self.iter()
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.iter_mut()
    }
}

impl<'a, K, V> VacantEntry<'a, K, V> for hash_map::VacantEntry<'a, K, V> {
    fn key(&self) -> &K {
        self.key()
    }

    fn insert(self, value: V) -> &'a mut V {
        self.insert(value)
    }
}

impl<'a, K, V> OccupiedEntry<'a, K, V> for hash_map::OccupiedEntry<'a, K, V> {
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
