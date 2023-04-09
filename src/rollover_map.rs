use std::{array, collections::HashMap, iter, mem, slice};

use arrayvec::ArrayVec;

use crate::{Entry, GenericMap, OccupiedEntry, VacantEntry};

use self::take_iter::TakeIter;

mod take_iter;

pub struct RolloverMap<K, V, const N: usize = 1, M = HashMap<K, V>> {
    stack_keys: ArrayVec<K, N>,
    stack_values: [V; N],
    heap: M,
}

impl<K, V, const N: usize, M: Default> Default for RolloverMap<K, V, N, M>
where
    [V; N]: Default,
{
    fn default() -> Self {
        Self {
            stack_keys: ArrayVec::new(),
            stack_values: Default::default(),
            heap: Default::default(),
        }
    }
}

impl<K, V, const N: usize, M: IntoIterator<Item = (K, V)>> IntoIterator
    for RolloverMap<K, V, N, M>
{
    type Item = (K, V);
    type IntoIter =
        iter::Chain<iter::Zip<arrayvec::IntoIter<K, N>, array::IntoIter<V, N>>, M::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        self.stack_keys
            .into_iter()
            .zip(self.stack_values.into_iter())
            .chain(self.heap.into_iter())
    }
}

pub type Iter<'a, K, V, I> = iter::Chain<iter::Zip<slice::Iter<'a, K>, slice::Iter<'a, V>>, I>;
pub type IterMut<'a, K, V, I> =
    iter::Chain<iter::Zip<slice::Iter<'a, K>, slice::IterMut<'a, V>>, I>;

impl<'a, K: Eq, V: Default, const N: usize, M: GenericMap<K, V>> IntoIterator
    for &'a RolloverMap<K, V, N, M>
where
    [V; N]: Default,
{
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V, M::Iter<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, K: Eq, V: Default, const N: usize, M: GenericMap<K, V>> IntoIterator
    for &'a mut RolloverMap<K, V, N, M>
where
    [V; N]: Default,
{
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V, M::IterMut<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<K: Eq, V: Default, const N: usize, M: GenericMap<K, V>> Extend<(K, V)>
    for RolloverMap<K, V, N, M>
{
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        let mut iter = iter.into_iter();
        if self.heap.is_empty() {
            for ((key, value), val) in
                (&mut iter).zip(self.stack_values[self.stack_keys.len()..].iter_mut())
            {
                self.stack_keys.push(key);
                *val = value;
            }
        }
        self.heap.extend(iter);
        if !self.heap.is_empty() {
            self.heap.extend(
                self.stack_keys
                    .drain(..)
                    .zip(self.stack_values.iter_mut().map(mem::take)),
            );
        }
    }
}

impl<K: Eq, V: Default, const N: usize, M: GenericMap<K, V>> GenericMap<K, V>
    for RolloverMap<K, V, N, M>
where
    [V; N]: Default,
{
    type Iter<'a> = Iter<'a, K, V, M::Iter<'a>>
    where
        Self: 'a;

    type IterMut<'a> = iter::Chain<
        iter::Zip<slice::Iter<'a, K>, slice::IterMut<'a, V>>,
        M::IterMut<'a>,
    >
    where
        Self: 'a;

    type DrainIter<'a> = iter::Chain<
        iter::Zip<arrayvec::Drain<'a, K, N>, TakeIter<'a, V>>,
        M::DrainIter<'a>,
    >
    where
        Self: 'a;

    type VacEntry<'a> = VacEntry<'a, K, V, N, M, M::VacEntry<'a>>
    where
        Self: 'a;

    type OccupEntry<'a> = OccupEntry<'a, K, V, N, M, M::OccupEntry<'a>>
    where
        Self: 'a;

    fn new() -> Self {
        Self::default()
    }

    fn len(&self) -> usize {
        self.stack_keys.len() + self.heap.len()
    }

    fn is_empty(&self) -> bool {
        self.stack_keys.is_empty() && self.heap.is_empty()
    }

    fn get(&self, key: &K) -> Option<&V> {
        for (k, v) in self.stack_keys.iter().zip(self.stack_values.iter()) {
            if k == key {
                return Some(v);
            }
        }
        self.heap.get(key)
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        for (k, v) in self.stack_keys.iter().zip(self.stack_values.iter_mut()) {
            if k == key {
                return Some(v);
            }
        }
        self.heap.get_mut(key)
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        for (k, v) in self.stack_keys.iter_mut().zip(self.stack_values.iter_mut()) {
            if k == &key {
                return Some(mem::replace(v, value));
            }
        }
        if self.heap.is_empty() {
            if self.stack_keys.len() < N {
                self.stack_keys.push(key);
                self.stack_values[self.stack_keys.len() - 1] = value;
                return None;
            }
            for (k, v) in self
                .stack_keys
                .drain(..)
                .zip(self.stack_values.iter_mut().map(mem::take))
            {
                self.heap.insert(k, v);
            }
        }
        self.heap.insert(key, value)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        for (i, (k, v)) in self
            .stack_keys
            .iter()
            .zip(self.stack_values.iter_mut())
            .enumerate()
        {
            if k == key {
                self.stack_keys.remove(i);
                let result = mem::take(v);
                self.stack_values[i..].rotate_left(1);
                return Some(result);
            }
        }
        let result = self.heap.remove(key);
        if self.heap.len() == N {
            for ((k, v), val) in self.heap.drain().zip(self.stack_values.iter_mut()) {
                self.stack_keys.push(k);
                *val = v;
            }
        }
        result
    }

    fn drain(&mut self) -> Self::DrainIter<'_> {
        let nelems = self.stack_keys.len();
        self.stack_keys
            .drain(..)
            .zip(TakeIter::new(&mut self.stack_values[..nelems]))
            .chain(self.heap.drain())
    }

    fn entry(&mut self, key: K) -> Entry<Self::VacEntry<'_>, Self::OccupEntry<'_>> {
        for (i, (k, v)) in self
            .stack_keys
            .iter()
            .zip(self.stack_values.iter_mut())
            .enumerate()
        {
            if k == &key {
                return Entry::Occupied(OccupEntry::Stack {
                    index: i,
                    key: k,
                    value: v,
                    stack_keys: &mut self.stack_keys,
                    stack_values: &mut self.stack_values,
                });
            }
        }
        if self.heap.is_empty() {
            return Entry::Vacant(VacEntry::Stack {
                key,
                stack_keys: &mut self.stack_keys,
                stack_values: &mut self.stack_values,
                heap: &mut self.heap,
            });
        }
        let heap_ptr = &mut self.heap as *mut M;
        match self.heap.entry(key) {
            Entry::Vacant(v) => Entry::Vacant(VacEntry::Heap(v)),
            Entry::Occupied(o) => Entry::Occupied(OccupEntry::Heap {
                heap_ptr,
                entry: o,
                stack_keys: &mut self.stack_keys,
                stack_values: &mut self.stack_values,
            }),
        }
    }

    fn iter(&self) -> Self::Iter<'_> {
        self.stack_keys
            .iter()
            .zip(self.stack_values.iter())
            .chain(self.heap.iter())
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.stack_keys
            .iter()
            .zip(self.stack_values.iter_mut())
            .chain(self.heap.iter_mut())
    }
}

pub enum VacEntry<'a, K, V, const N: usize, M, E> {
    Stack {
        key: K,
        stack_keys: &'a mut ArrayVec<K, N>,
        stack_values: &'a mut [V],
        heap: &'a mut M,
    },
    Heap(E),
}

impl<'a, K, V: Default, const N: usize, M: GenericMap<K, V>, E: VacantEntry<'a, K, V>>
    VacantEntry<'a, K, V> for VacEntry<'a, K, V, N, M, E>
{
    fn key(&self) -> &K {
        match self {
            VacEntry::Stack { key, .. } => key,
            VacEntry::Heap(entry) => entry.key(),
        }
    }

    fn insert(self, value: V) -> &'a mut V {
        match self {
            VacEntry::Stack {
                key,
                stack_keys,
                stack_values,
                heap,
            } => {
                if stack_keys.len() < N {
                    stack_keys.push(key);
                    let v = &mut stack_values[stack_keys.len() - 1];
                    *v = value;
                    v
                } else {
                    heap.extend(
                        stack_keys
                            .drain(..)
                            .zip(stack_values.iter_mut().map(mem::take)),
                    );
                    match heap.entry(key) {
                        Entry::Vacant(vac) => vac.insert(value),
                        Entry::Occupied(_) => panic!("Bad map implementation"),
                    }
                }
            }
            VacEntry::Heap(entry) => entry.insert(value),
        }
    }
}

pub enum OccupEntry<'a, K, V, const N: usize, M, E> {
    Stack {
        index: usize,
        stack_keys: &'a mut ArrayVec<K, N>,
        stack_values: &'a mut [V],
        key: *const K,
        value: *mut V,
    },
    Heap {
        entry: E,
        stack_keys: &'a mut ArrayVec<K, N>,
        stack_values: &'a mut [V],
        heap_ptr: *mut M,
    },
}

impl<'a, K, V: Default, const N: usize, M: GenericMap<K, V>, E: OccupiedEntry<'a, K, V>>
    OccupiedEntry<'a, K, V> for OccupEntry<'a, K, V, N, M, E>
{
    fn key(&self) -> &K {
        match self {
            OccupEntry::Stack { key, .. } => unsafe { &**key },
            OccupEntry::Heap { entry, .. } => entry.key(),
        }
    }

    fn get(&self) -> &V {
        match self {
            OccupEntry::Stack { value, .. } => unsafe { &**value },
            OccupEntry::Heap { entry, .. } => entry.get(),
        }
    }

    fn get_mut(&mut self) -> &mut V {
        match self {
            OccupEntry::Stack { value, .. } => unsafe { &mut **value },
            OccupEntry::Heap { entry, .. } => entry.get_mut(),
        }
    }

    fn insert(&mut self, new_value: V) -> V {
        match self {
            OccupEntry::Stack { value, .. } => unsafe { mem::replace(&mut **value, new_value) },
            OccupEntry::Heap { entry, .. } => entry.insert(new_value),
        }
    }

    fn remove(self) -> V {
        match self {
            OccupEntry::Stack {
                index,
                stack_keys,
                stack_values,
                ..
            } => {
                stack_keys.remove(index);
                let result = mem::take(&mut stack_values[index]);
                stack_values[index..].rotate_left(1);
                result
            }
            OccupEntry::Heap {
                entry,
                stack_keys,
                stack_values,
                heap_ptr,
            } => {
                let result = entry.remove();
                let heap = unsafe { &mut *heap_ptr };
                if heap.len() == N {
                    for ((k, v), val) in heap.drain().zip(stack_values.iter_mut()) {
                        stack_keys.push(k);
                        *val = v;
                    }
                }
                result
            }
        }
    }

    fn into_mut(self) -> &'a mut V {
        match self {
            OccupEntry::Stack { value, .. } => unsafe { &mut *value },
            OccupEntry::Heap { entry, .. } => entry.into_mut(),
        }
    }
}