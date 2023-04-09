use std::collections::{hash_map, HashMap};
use std::hash::Hash;
use std::{iter, mem};

use crate::{GenericMap, OccupiedEntry, VacantEntry};

use self::comparator::{Comparator, Max, Min};
use self::indexed_heap::{Index, IndexedHeap};

mod comparator;
mod indexed_heap;

pub struct HashedHeap<K, V, C> {
    map: HashMap<K, (V, Index)>,
    heap: IndexedHeap<K, C>,
}

pub type HashedMaxHeap<K, V> = HashedHeap<K, V, Max<K>>;
pub type HashedMinHeap<K, V> = HashedHeap<K, V, Min<K>>;

impl<K, V> HashedMaxHeap<K, V> {
    pub fn max(&self) -> Option<(&K, &V)>
    where
        K: Eq + Hash,
    {
        self.peek()
    }
}

impl<K, V> HashedMinHeap<K, V> {
    pub fn min(&self) -> Option<(&K, &V)>
    where
        K: Eq + Hash,
    {
        self.peek()
    }
}

impl<K, V, C> HashedHeap<K, V, C> {
    pub fn new() -> Self
    where
        C: Default,
    {
        Self {
            map: HashMap::new(),
            heap: IndexedHeap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn contains_key(&self, key: &K) -> bool
    where
        K: Eq + Hash,
    {
        self.map.contains_key(key)
    }

    pub fn get(&self, key: &K) -> Option<&V>
    where
        K: Eq + Hash,
    {
        self.map.get(key).map(|(v, _)| v)
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V>
    where
        K: Eq + Hash,
    {
        self.map.get_mut(key).map(|(v, _)| v)
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V>
    where
        K: Eq + Hash + Clone,
        C: Comparator<K>,
    {
        match self.map.entry(key) {
            hash_map::Entry::Occupied(occ) => Some(mem::replace(&mut occ.into_mut().0, value)),
            hash_map::Entry::Vacant(vac) => {
                let (index, changes) = self.heap.insert(vac.key().clone());
                vac.insert((value, index));
                for (new_index, k) in changes {
                    self.map.get_mut(k).unwrap().1 = new_index;
                }
                None
            }
        }
    }

    pub fn remove(&mut self, key: &K) -> Option<V>
    where
        K: Eq + Hash,
        C: Comparator<K>,
    {
        let (result, index) = self.map.remove(key)?;
        let (k, changes) = self.heap.remove(index);
        assert!(k == *key);
        for (new_index, k) in changes {
            self.map.get_mut(&k).unwrap().1 = new_index;
        }
        Some(result)
    }

    pub fn drain(&mut self) -> DrainIter<'_, K, V> {
        self.heap.clear();
        self.map.drain().map(|(k, (v, _))| (k, v))
    }

    pub fn entry(&mut self, key: K) -> crate::Entry<VacEntry<'_, K, V, C>, OccupEntry<'_, K, V, C>>
    where
        K: Eq + Hash,
    {
        let map = &mut self.map as *mut HashMap<K, (V, Index)>;
        match self.map.entry(key) {
            hash_map::Entry::Occupied(occ) => crate::Entry::Occupied(Entry {
                map,
                heap: &mut self.heap,
                entry: occ,
            }),
            hash_map::Entry::Vacant(vac) => crate::Entry::Vacant(Entry {
                map,
                heap: &mut self.heap,
                entry: vac,
            }),
        }
    }

    pub fn iter(&self) -> Iter<'_, K, V> {
        self.map.iter().map(|(k, (v, _))| (k, v))
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        self.map.iter_mut().map(|(k, (v, _))| (k, v))
    }

    fn peek(&self) -> Option<(&K, &V)>
    where
        K: Eq + Hash,
    {
        self.heap.peek().map(|k| {
            let (v, _) = self.map.get(k).unwrap();
            (k, v)
        })
    }
}

impl<K, V, C: Default> Default for HashedHeap<K, V, C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V, C> IntoIterator for HashedHeap<K, V, C> {
    type Item = (K, V);
    type IntoIter = iter::Map<hash_map::IntoIter<K, (V, Index)>, fn((K, (V, Index))) -> (K, V)>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter().map(|(k, (v, _))| (k, v))
    }
}

impl<K: Eq + Hash + Clone, V, C: Comparator<K>> Extend<(K, V)> for HashedHeap<K, V, C> {
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        for (k, v) in iter {
            self.insert(k, v);
        }
    }
}

pub type Iter<'a, K, V> =
    iter::Map<hash_map::Iter<'a, K, (V, Index)>, fn((&'a K, &'a (V, Index))) -> (&'a K, &'a V)>;

pub type IterMut<'a, K, V> = iter::Map<
    hash_map::IterMut<'a, K, (V, Index)>,
    fn((&'a K, &'a mut (V, Index))) -> (&'a K, &'a mut V),
>;

pub type DrainIter<'a, K, V> =
    iter::Map<hash_map::Drain<'a, K, (V, Index)>, fn((K, (V, Index))) -> (K, V)>;

impl<K: Eq + Hash + Clone, V, C: Comparator<K>> GenericMap<K, V> for HashedHeap<K, V, C> {
    type Iter<'a> = Iter<'a, K, V>
    where
        K: 'a,
        V: 'a,
        Self: 'a;

    type IterMut<'a> = IterMut<'a, K, V>
    where
        K: 'a,
        V: 'a,
        Self: 'a;

    type DrainIter<'a> = DrainIter<'a, K, V>
    where
        Self: 'a;

    type VacEntry<'a> = VacEntry<'a, K, V, C>
    where
        Self: 'a;

    type OccupEntry<'a> = OccupEntry<'a, K, V, C>
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
        self.drain()
    }

    fn entry(&mut self, key: K) -> crate::Entry<Self::VacEntry<'_>, Self::OccupEntry<'_>> {
        self.entry(key)
    }

    fn iter(&self) -> Self::Iter<'_> {
        self.iter()
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.iter_mut()
    }
}

pub struct Entry<'a, K, V, C, E> {
    map: *mut HashMap<K, (V, Index)>,
    heap: &'a mut IndexedHeap<K, C>,
    entry: E,
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
