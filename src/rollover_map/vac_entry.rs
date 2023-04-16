use std::mem;

use arrayvec::ArrayVec;

use crate::{Entry, GenericMap, VacantEntry};

pub struct VacEntry<'a, K, V, const N: usize, M, E>(VacEntryInner<'a, K, V, N, M, E>);

impl<'a, K, V, const N: usize, M, E> VacEntry<'a, K, V, N, M, E> {
    pub(super) fn stack(
        key: K,
        stack_keys: &'a mut ArrayVec<K, N>,
        stack_values: &'a mut [V; N],
        heap: &'a mut M,
    ) -> Self {
        Self(VacEntryInner::Stack {
            key,
            stack_keys,
            stack_values,
            heap,
        })
    }

    pub(super) fn heap(entry: E) -> Self {
        Self(VacEntryInner::Heap(entry))
    }

    pub fn key(&self) -> &K
    where
        E: VacantEntry<'a, K, V>,
    {
        self.0.key()
    }

    pub fn insert(self, value: V) -> &'a mut V
    where
        V: Default,
        M: GenericMap<K = K, V = V>,
        E: VacantEntry<'a, K, V>,
    {
        self.0.insert(value)
    }
}

enum VacEntryInner<'a, K, V, const N: usize, M, E> {
    Stack {
        key: K,
        stack_keys: &'a mut ArrayVec<K, N>,
        stack_values: &'a mut [V],
        heap: &'a mut M,
    },
    Heap(E),
}

impl<'a, K, V, const N: usize, M, E> VacEntryInner<'a, K, V, N, M, E> {
    pub fn key(&self) -> &K
    where
        E: VacantEntry<'a, K, V>,
    {
        match self {
            VacEntryInner::Stack { key, .. } => key,
            VacEntryInner::Heap(entry) => entry.key(),
        }
    }

    pub fn insert(self, value: V) -> &'a mut V
    where
        V: Default,
        M: GenericMap<K = K, V = V>,
        E: VacantEntry<'a, K, V>,
    {
        match self {
            VacEntryInner::Stack {
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
            VacEntryInner::Heap(entry) => entry.insert(value),
        }
    }
}

impl<'a, K, V: Default, const N: usize, M: GenericMap<K = K, V = V>, E: VacantEntry<'a, K, V>>
    VacantEntry<'a, K, V> for VacEntry<'a, K, V, N, M, E>
{
    fn key(&self) -> &K {
        self.key()
    }

    fn insert(self, value: V) -> &'a mut V {
        self.insert(value)
    }
}
