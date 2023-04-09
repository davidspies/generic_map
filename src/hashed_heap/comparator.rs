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
}

impl<T> Comparator<T> for Max<T>
where
    T: PartialOrd,
{
    fn favors(&self, a: &T, b: &T) -> bool {
        a > b
    }
}

impl<T> Comparator<T> for Min<T>
where
    T: PartialOrd,
{
    fn favors(&self, a: &T, b: &T) -> bool {
        a < b
    }
}
