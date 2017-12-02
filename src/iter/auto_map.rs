use std::fmt;
use std::iter::FromIterator;
use std::marker::PhantomData;

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct AutoMapInto<B: From<I::Item>, I: Iterator> {
    iter: I,
    _b_marker: PhantomData<B>,
}

impl<B: From<I::Item>, I: fmt::Debug + Iterator> fmt::Debug for AutoMapInto<B, I> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("AutoMapInto")
            .field("iter", &self.iter)
            .finish()
    }
}

impl<B: From<I::Item>, I: Iterator> Iterator for AutoMapInto<B, I> {
    type Item = B;

    #[inline]
    fn next(&mut self) -> Option<B> {
        self.iter.next().map(|i| i.into())
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    fn fold<Acc, G>(self, init: Acc, mut g: G) -> Acc
        where G: FnMut(Acc, Self::Item) -> Acc
    {
        self.iter.fold(init, move |acc, elt| g(acc, elt.into()))
    }
}

impl<B: From<I::Item>, I: DoubleEndedIterator> DoubleEndedIterator for AutoMapInto<B,I> {
    #[inline]
    fn next_back(&mut self) -> Option<B> {
        self.iter.next_back().map(|i| i.into())
    }
}


pub trait AutoMap : Iterator
{
    fn auto_map<A, B>(self) -> AutoMapInto<B, Self>
        where Self: Sized + Iterator<Item=A>,
            B: From<A>
    {
        AutoMapInto{ iter: self, _b_marker: PhantomData }
    }

    fn auto_map_collect<A, B, V>(self) -> V
        where Self: Sized + Iterator<Item=A>,
            B: From<A>,
            V: FromIterator<B>
    {
        self.auto_map().collect()
    }
}

impl<T: ?Sized> AutoMap for T
    where T: Iterator {}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Eq, PartialEq)]
    struct Foo(u32);

    impl From<u32> for Foo {
        fn from(other: u32) -> Foo {
            Foo(other)
        }
    }

    #[test]
    fn map() {
        let ints = 1_u32..4;
        let foos: Vec<Foo> = ints.auto_map().collect();

        assert_eq!(foos, vec![Foo(1), Foo(2), Foo(3)]);
    }

    #[test]
    fn collect() {
        let ints = 1_u32..4;
        let foos: Vec<Foo> = ints.clone().auto_map().collect();
        let collected: Vec<Foo> = ints.auto_map_collect();

        assert_eq!(foos, collected);
    }
}