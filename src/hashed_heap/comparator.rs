use std::marker::PhantomData;

pub struct Max<T>(PhantomData<T>);
impl<T> Default for Max<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

pub struct Min<T>(PhantomData<T>);
impl<T> Default for Min<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

pub trait Comparator<T>: Default {
    fn favors(&self, a: &T, b: &T) -> bool;
    fn favored<'a>(&self, a: &'a T, b: &'a T) -> &'a T {
        if self.favors(b, a) {
            b
        } else {
            a
        }
    }
}

impl<T: PartialOrd> Comparator<T> for Max<T> {
    fn favors(&self, a: &T, b: &T) -> bool {
        a > b
    }
}

impl<T: PartialOrd> Comparator<T> for Min<T> {
    fn favors(&self, a: &T, b: &T) -> bool {
        a < b
    }
}
