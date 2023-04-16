use std::collections::{hash_map, HashMap};
use std::hash::Hash;
use std::mem;

use crate::{OccupiedEntry, VacantEntry};

use super::comparator::Comparator;
use super::indexed_heap::{Index, IndexedHeap};

pub struct Entry<'a, K, V, C, E> {
    map: *mut HashMap<K, (V, Index)>,
    heap: &'a mut IndexedHeap<K, C>,
    entry: E,
}

impl<'a, K, V, C, E> Entry<'a, K, V, C, E> {
    pub(super) unsafe fn new(
        map: *mut HashMap<K, (V, Index)>,
        heap: &'a mut IndexedHeap<K, C>,
        entry: E,
    ) -> Self {
        Self { map, heap, entry }
    }
}

pub type VacEntry<'a, K, V, C> = Entry<'a, K, V, C, hash_map::VacantEntry<'a, K, (V, Index)>>;
pub type OccupEntry<'a, K, V, C> = Entry<'a, K, V, C, hash_map::OccupiedEntry<'a, K, (V, Index)>>;

impl<'a, K, V, C> VacEntry<'a, K, V, C> {
    pub fn key(&self) -> &K {
        self.entry.key()
    }

    pub fn insert(self, value: V) -> &'a mut V
    where
        K: Eq + Hash + Clone,
        C: Comparator<K>,
    {
        let (index, changed_indices) = self.heap.insert(self.entry.key().clone());
        let result = &mut self.entry.insert((value, index)).0;
        let map = unsafe { &mut *self.map };
        for (new_index, k) in changed_indices {
            map.get_mut(&k).unwrap().1 = new_index;
        }
        result
    }
}

impl<'a, K: Eq + Hash + Clone, V, C: Comparator<K>> VacantEntry<'a, K, V>
    for VacEntry<'a, K, V, C>
{
    fn key(&self) -> &K {
        self.key()
    }

    fn insert(self, value: V) -> &'a mut V {
        self.insert(value)
    }
}

impl<'a, K, V, C> OccupEntry<'a, K, V, C> {
    pub fn key(&self) -> &K {
        self.entry.key()
    }

    pub fn insert(&mut self, value: V) -> V {
        mem::replace(&mut self.entry.get_mut().0, value)
    }

    pub fn remove(self) -> V
    where
        K: Eq + Hash,
        C: Comparator<K>,
    {
        let (k1, (result, index)) = self.entry.remove_entry();
        let (k2, changed_indices) = self.heap.remove(index);
        assert!(k1 == k2);
        let map = unsafe { &mut *self.map };
        for (new_index, k) in changed_indices {
            map.get_mut(&k).unwrap().1 = new_index;
        }
        result
    }

    pub fn get(&self) -> &V {
        &self.entry.get().0
    }

    pub fn get_mut(&mut self) -> &mut V {
        &mut self.entry.get_mut().0
    }

    pub fn into_mut(self) -> &'a mut V {
        &mut self.entry.into_mut().0
    }
}

impl<'a, K: Eq + Hash, V, C: Comparator<K>> OccupiedEntry<'a, K, V> for OccupEntry<'a, K, V, C> {
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
