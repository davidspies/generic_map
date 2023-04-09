mod btree_map_impl;
mod hash_map_impl;

pub trait GenericMap<K, V>: IntoIterator<Item = (K, V)>
where
    for<'a> &'a Self: IntoIterator<Item = (&'a K, &'a V)>,
    for<'a> &'a mut Self: IntoIterator<Item = (&'a K, &'a mut V)>,
{
    type DrainIter<'a>: Iterator<Item = (K, V)> + 'a
    where
        Self: 'a;
    type VacEntry<'a>: VacantEntry<'a, K, V>
    where
        Self: 'a;
    type OccupEntry<'a>: OccupiedEntry<'a, K, V>
    where
        Self: 'a;

    fn get(&self, key: &K) -> Option<&V>;
    fn get_mut(&mut self, key: &K) -> Option<&mut V>;
    fn insert(&mut self, key: K, value: V) -> Option<V>;
    fn remove(&mut self, key: &K) -> Option<V>;
    fn drain(&mut self) -> Self::DrainIter<'_>;
    fn entry(&mut self, key: K) -> Entry<Self::VacEntry<'_>, Self::OccupEntry<'_>>;
}

pub enum Entry<V, O> {
    Vacant(V),
    Occupied(O),
}

pub trait VacantEntry<'a, K, V> {
    fn insert(self, value: V) -> &'a mut V;
}

pub trait OccupiedEntry<'a, K, V> {
    fn insert(&mut self, value: V) -> V;
    fn remove(self) -> V;
    fn get(&self) -> &V;
    fn get_mut(&mut self) -> &mut V;
    fn into_mut(self) -> &'a mut V;
}
