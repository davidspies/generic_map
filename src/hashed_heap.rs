use std::collections::{hash_map, HashMap};
use std::hash::Hash;
use std::{iter, mem};

use crate::clear::Clear;
use crate::drain::Drain;
use crate::GenericMap;

use self::comparator::{Comparator, Max, Min};
use self::indexed_heap::{Index, IndexedHeap};

pub use self::entry::{Entry, OccupEntry, VacEntry};

mod comparator;
mod entry;
mod indexed_heap;

pub struct HashedHeap<K, V, C> {
    map: HashMap<K, (V, Index)>,
    heap: IndexedHeap<K, C>,
}

pub type HashedMaxHeap<K, V> = HashedHeap<K, V, Max<K>>;
pub type HashedMinHeap<K, V> = HashedHeap<K, V, Min<K>>;

impl<K, V> HashedMaxHeap<K, V> {
    pub fn max_key(&self) -> Option<&K> {
        self.peek()
    }
}

impl<K, V> HashedMinHeap<K, V> {
    pub fn min_key(&self) -> Option<&K> {
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
            hash_map::Entry::Occupied(occ) => {
                crate::Entry::Occupied(unsafe { Entry::new(map, &mut self.heap, occ) })
            }
            hash_map::Entry::Vacant(vac) => {
                crate::Entry::Vacant(unsafe { Entry::new(map, &mut self.heap, vac) })
            }
        }
    }

    pub fn iter(&self) -> Iter<'_, K, V> {
        self.map.iter().map(|(k, (v, _))| (k, v))
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        self.map.iter_mut().map(|(k, (v, _))| (k, v))
    }

    fn peek(&self) -> Option<&K> {
        self.heap.peek()
    }
}

impl<K, V, C: Default> Default for HashedHeap<K, V, C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V, C> Clear for HashedHeap<K, V, C> {
    fn clear(&mut self) {
        self.map.clear();
        self.heap.clear();
    }
}

impl<K, V, C> Drain for HashedHeap<K, V, C> {
    type Output<'a> = DrainIter<'a, K, V>
    where
        Self: 'a;

    fn drain(&mut self) -> Self::Output<'_> {
        self.drain()
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

impl<K: Eq + Hash + Clone, V, C: Comparator<K>> GenericMap for HashedHeap<K, V, C> {
    type K = K;
    type V = V;
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
