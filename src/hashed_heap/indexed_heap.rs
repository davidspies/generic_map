use super::comparator::Comparator;

pub struct IndexedHeap<T, C> {
    data: Vec<T>,
    changed_indices_scratch: Vec<Index>,
    compare: C,
}

impl<T, C: Default> Default for IndexedHeap<T, C> {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            changed_indices_scratch: Vec::new(),
            compare: C::default(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Index(usize);

impl<T, C> std::ops::Index<Index> for IndexedHeap<T, C> {
    type Output = T;

    fn index(&self, index: Index) -> &Self::Output {
        &self.data[index.0]
    }
}

impl<T, C> IndexedHeap<T, C> {
    pub fn new() -> Self
    where
        C: Default,
    {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn insert(&mut self, value: T) -> (Index, impl Iterator<Item = (Index, &'_ T)> + '_)
    where
        C: Comparator<T>,
    {
        let mut index = Index(self.data.len());
        self.data.push(value);
        self.sift_up(&mut index);
        (
            index,
            self.changed_indices_scratch
                .drain(..)
                .map(|i| (i, &self.data[i.0])),
        )
    }

    pub fn remove(&mut self, mut index: Index) -> (T, impl Iterator<Item = (Index, &'_ T)> + '_)
    where
        C: Comparator<T>,
    {
        let last_index = Index(self.data.len() - 1);
        self.swap(index, last_index);
        let value = self.data.pop().unwrap();
        if index != last_index {
            self.sift_up(&mut index);
            self.sift_down(&mut index);
            self.changed_indices_scratch.push(index);
        }
        (
            value,
            self.changed_indices_scratch
                .drain(..)
                .map(|i| (i, &self.data[i.0])),
        )
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.get(0)
    }

    fn swap(&mut self, a: Index, b: Index) {
        self.data.swap(a.0, b.0);
    }

    fn sift_up(&mut self, index: &mut Index)
    where
        C: Comparator<T>,
    {
        while let Some(parent) = parent(*index) {
            if self.compare.favors(&self[*index], &self[parent]) {
                self.swap(parent, *index);
                self.changed_indices_scratch.push(*index);
                *index = parent;
            } else {
                break;
            }
        }
    }

    fn sift_down(&mut self, index: &mut Index)
    where
        C: Comparator<T>,
    {
        loop {
            let (left, right) = children(*index);
            let mut favored_child = left;
            if right.0 < self.data.len() {
                if self.compare.favors(&self[right], &self[left]) {
                    favored_child = right;
                }
            }
            if favored_child.0 < self.data.len() {
                if self.compare.favors(&self[favored_child], &self[*index]) {
                    self.swap(favored_child, *index);
                    self.changed_indices_scratch.push(*index);
                    *index = favored_child;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }
}

fn parent(index: Index) -> Option<Index> {
    if index.0 == 0 {
        None
    } else {
        Some(Index((index.0 - 1) / 2))
    }
}

fn children(index: Index) -> (Index, Index) {
    (Index(index.0 * 2 + 1), Index(index.0 * 2 + 2))
}
