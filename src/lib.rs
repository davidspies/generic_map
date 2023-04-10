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

pub trait GenericMap:
    Default + Extend<(Self::K, Self::V)> + IntoIterator<Item = (Self::K, Self::V)>
{
    type K;
    type V;
    type Iter<'a>: Iterator<Item = (&'a Self::K, &'a Self::V)>
    where
        Self::K: 'a,
        Self::V: 'a,
        Self: 'a;
    type IterMut<'a>: Iterator<Item = (&'a Self::K, &'a mut Self::V)>
    where
        Self::K: 'a,
        Self::V: 'a,
        Self: 'a;
    type DrainIter<'a>: Iterator<Item = (Self::K, Self::V)>
    where
        Self: 'a;
    type VacEntry<'a>: VacantEntry<'a, Self::K, Self::V>
    where
        Self: 'a;
    type OccupEntry<'a>: OccupiedEntry<'a, Self::K, Self::V>
    where
        Self: 'a;

    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn contains_key(&self, key: &Self::K) -> bool;
    fn get(&self, key: &Self::K) -> Option<&Self::V>;
    fn get_mut(&mut self, key: &Self::K) -> Option<&mut Self::V>;
    fn insert(&mut self, key: Self::K, value: Self::V) -> Option<Self::V>;
    fn remove(&mut self, key: &Self::K) -> Option<Self::V>;
    fn drain(&mut self) -> Self::DrainIter<'_>;
    fn entry(&mut self, key: Self::K) -> Entry<Self::VacEntry<'_>, Self::OccupEntry<'_>>;
    fn iter(&self) -> Self::Iter<'_>;
    fn iter_mut(&mut self) -> Self::IterMut<'_>;
    fn remove_clearable(&mut self, key: &Self::K) -> bool
    where
        Self::V: Clear,
    {
        self.remove(key).is_some()
    }
    fn drain_or_remove(
        &mut self,
        key: &Self::K,
    ) -> Option<DrainOrRemove<<Self::V as Drain>::Output<'_>, Self::V>>
    where
        Self::V: Drain,
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
