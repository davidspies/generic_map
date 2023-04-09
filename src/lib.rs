use clear::Clear;
use drain::Drain;

pub use self::hashed_heap::{HashedMaxHeap, HashedMinHeap};
pub use self::rollover_map::RolloverMap;

mod btree_map_impl;
mod hash_map_impl;

pub mod clear;
pub mod drain;
pub mod hashed_heap;
pub mod rollover_map;

pub trait GenericMap<K, V>: Default + Extend<(K, V)> + IntoIterator<Item = (K, V)> {
    type Iter<'a>: Iterator<Item = (&'a K, &'a V)>
    where
        K: 'a,
        V: 'a,
        Self: 'a;
    type IterMut<'a>: Iterator<Item = (&'a K, &'a mut V)>
    where
        K: 'a,
        V: 'a,
        Self: 'a;
    type DrainIter<'a>: Iterator<Item = (K, V)>
    where
        Self: 'a;
    type VacEntry<'a>: VacantEntry<'a, K, V>
    where
        Self: 'a;
    type OccupEntry<'a>: OccupiedEntry<'a, K, V>
    where
        Self: 'a;

    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn contains_key(&self, key: &K) -> bool;
    fn get(&self, key: &K) -> Option<&V>;
    fn get_mut(&mut self, key: &K) -> Option<&mut V>;
    fn insert(&mut self, key: K, value: V) -> Option<V>;
    fn remove(&mut self, key: &K) -> Option<V>;
    fn drain(&mut self) -> Self::DrainIter<'_>;
    fn entry(&mut self, key: K) -> Entry<Self::VacEntry<'_>, Self::OccupEntry<'_>>;
    fn iter(&self) -> Self::Iter<'_>;
    fn iter_mut(&mut self) -> Self::IterMut<'_>;
    fn remove_clearable(&mut self, key: &K) -> bool
    where
        V: Clear,
    {
        self.remove(key).is_some()
    }
    fn drain_or_remove(&mut self, key: &K) -> Option<DrainOrRemove<V::Output<'_>, V>>
    where
        V: Drain,
    {
        self.remove(key).map(DrainOrRemove::Removed)
    }
}

pub enum DrainOrRemove<Drained, Removed> {
    Drained(Drained),
    Removed(Removed),
}

pub enum Entry<V, O> {
    Vacant(V),
    Occupied(O),
}

pub trait VacantEntry<'a, K, V> {
    fn key(&self) -> &K;
    fn insert(self, value: V) -> &'a mut V;
}

pub trait OccupiedEntry<'a, K, V> {
    fn key(&self) -> &K;
    fn insert(&mut self, value: V) -> V;
    fn remove(self) -> V;
    fn get(&self) -> &V;
    fn get_mut(&mut self) -> &mut V;
    fn into_mut(self) -> &'a mut V;
    fn remove_clearable(self)
    where
        V: Clear,
        Self: Sized,
    {
        self.remove();
    }
}
