use std::mem;

use arrayvec::ArrayVec;

use crate::{clear::Clear, GenericMap, OccupiedEntry};

pub struct OccupEntry<'a, K, V, const N: usize, M, E>(OccupEntryInner<'a, K, V, N, M, E>);

impl<'a, K, V, const N: usize, M, E> OccupEntry<'a, K, V, N, M, E> {
    pub(super) unsafe fn stack(
        index: usize,
        key: *const K,
        value: *mut V,
        stack_keys: &'a mut ArrayVec<K, N>,
        stack_values: &'a mut [V],
    ) -> Self {
        Self(OccupEntryInner::Stack {
            index,
            stack_keys,
            stack_values,
            key,
            value,
        })
    }

    pub(super) unsafe fn heap(
        heap_ptr: *mut M,
        entry: E,
        stack_keys: &'a mut ArrayVec<K, N>,
        stack_values: &'a mut [V],
    ) -> Self {
        Self(OccupEntryInner::Heap {
            entry,
            stack_keys,
            stack_values,
            heap_ptr,
        })
    }

    pub fn key(&self) -> &K
    where
        E: OccupiedEntry<'a, K, V>,
    {
        self.0.key()
    }

    pub fn get(&self) -> &V
    where
        E: OccupiedEntry<'a, K, V>,
    {
        self.0.get()
    }

    pub fn get_mut(&mut self) -> &mut V
    where
        E: OccupiedEntry<'a, K, V>,
    {
        self.0.get_mut()
    }

    pub fn insert(&mut self, new_value: V) -> V
    where
        E: OccupiedEntry<'a, K, V>,
    {
        self.0.insert(new_value)
    }

    pub fn remove(self) -> V
    where
        V: Default,
        M: GenericMap<K = K, V = V>,
        E: OccupiedEntry<'a, K, V>,
    {
        self.0.remove()
    }

    pub fn into_mut(self) -> &'a mut V
    where
        E: OccupiedEntry<'a, K, V>,
    {
        self.0.into_mut()
    }

    pub fn remove_clearable(self)
    where
        V: Clear,
        M: GenericMap<K = K, V = V>,
        E: OccupiedEntry<'a, K, V>,
    {
        self.0.remove_clearable()
    }
}

enum OccupEntryInner<'a, K, V, const N: usize, M, E> {
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

impl<'a, K, V, const N: usize, M, E> OccupEntryInner<'a, K, V, N, M, E> {
    pub fn key(&self) -> &K
    where
        E: OccupiedEntry<'a, K, V>,
    {
        match self {
            OccupEntryInner::Stack { key, .. } => unsafe { &**key },
            OccupEntryInner::Heap { entry, .. } => entry.key(),
        }
    }

    pub fn get(&self) -> &V
    where
        E: OccupiedEntry<'a, K, V>,
    {
        match self {
            OccupEntryInner::Stack { value, .. } => unsafe { &**value },
            OccupEntryInner::Heap { entry, .. } => entry.get(),
        }
    }

    pub fn get_mut(&mut self) -> &mut V
    where
        E: OccupiedEntry<'a, K, V>,
    {
        match self {
            OccupEntryInner::Stack { value, .. } => unsafe { &mut **value },
            OccupEntryInner::Heap { entry, .. } => entry.get_mut(),
        }
    }

    pub fn insert(&mut self, new_value: V) -> V
    where
        E: OccupiedEntry<'a, K, V>,
    {
        match self {
            OccupEntryInner::Stack { value, .. } => unsafe {
                mem::replace(&mut **value, new_value)
            },
            OccupEntryInner::Heap { entry, .. } => entry.insert(new_value),
        }
    }

    pub fn remove(self) -> V
    where
        V: Default,
        M: GenericMap<K = K, V = V>,
        E: OccupiedEntry<'a, K, V>,
    {
        match self {
            OccupEntryInner::Stack {
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
            OccupEntryInner::Heap {
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

    pub fn into_mut(self) -> &'a mut V
    where
        E: OccupiedEntry<'a, K, V>,
    {
        match self {
            OccupEntryInner::Stack { value, .. } => unsafe { &mut *value },
            OccupEntryInner::Heap { entry, .. } => entry.into_mut(),
        }
    }

    pub fn remove_clearable(self)
    where
        V: Clear,
        M: GenericMap<K = K, V = V>,
        E: OccupiedEntry<'a, K, V>,
    {
        match self {
            OccupEntryInner::Stack {
                index,
                stack_keys,
                stack_values,
                ..
            } => {
                stack_keys.remove(index);
                stack_values[index].clear();
                stack_values[index..].rotate_left(1);
            }
            OccupEntryInner::Heap {
                entry,
                stack_keys,
                stack_values,
                heap_ptr,
            } => {
                entry.remove_clearable();
                let heap = unsafe { &mut *heap_ptr };
                if heap.len() == N {
                    for ((k, v), val) in heap.drain().zip(stack_values.iter_mut()) {
                        stack_keys.push(k);
                        *val = v;
                    }
                }
            }
        }
    }
}

impl<'a, K, V, const N: usize, M, E> OccupiedEntry<'a, K, V> for OccupEntry<'a, K, V, N, M, E>
where
    V: Default,
    M: GenericMap<K = K, V = V>,
    E: OccupiedEntry<'a, K, V>,
{
    fn key(&self) -> &K {
        self.key()
    }

    fn get(&self) -> &V {
        self.get()
    }

    fn get_mut(&mut self) -> &mut V {
        self.get_mut()
    }

    fn insert(&mut self, new_value: V) -> V {
        self.insert(new_value)
    }

    fn remove(self) -> V {
        self.remove()
    }

    fn into_mut(self) -> &'a mut V {
        self.into_mut()
    }

    fn remove_clearable(self)
    where
        V: Clear,
    {
        self.remove_clearable()
    }
}
