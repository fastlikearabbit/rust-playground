#![forbid(unsafe_code)]

pub struct LazyCycle<I>
where
    I: Iterator,
{
    iter: I,
}

impl<I: Iterator> Iterator for LazyCycle<I> {
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct Extract<I: Iterator> {
    iter: I,
}

impl<I: Iterator> Iterator for Extract<I> {
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct Tee<I>
where
    I: Iterator,
    I::Item: Clone,
{
    iter: I,
}

impl<I> Iterator for Tee<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
////////////////////////////////////////////////////////////////////////////////

pub struct GroupBy<I, F, V>
where
    I: Iterator,
    F: FnMut(&I::Item) -> V,
    V: Eq,
{
    iter: I,
    f: F,
    value: V,
}

impl<I, F, V> Iterator for GroupBy<I, F, V>
where
    I: Iterator,
    F: FnMut(&I::Item) -> V,
    V: Eq,
{
    type Item = (V, std::vec::IntoIter<I::Item>);

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

////////////////////////////////////////////////////////////////////////////////

pub trait ExtendedIterator: Iterator {
    fn lazy_cycle(self) -> LazyCycle<Self>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        // TODO: your code goes here.
        // use .by_ref() to take the iterator by &mut  
        unimplemented!()
    }

    fn extract(mut self, index: usize) -> (Option<Self::Item>, Extract<Self>)
    where
        Self: Sized,
    {
        // TODO: your code goes here.
        unimplemented!()
    }

    fn tee(self) -> (Tee<Self>, Tee<Self>)
    where
        Self: Sized,
        Self::Item: Clone,
    {
        // TODO: your code goes here.
        unimplemented!()
    }

    fn group_by<F, V>(self, func: F) -> GroupBy<Self, F, V>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> V,
        V: Eq,
    {
        // TODO: your code goes here.
        unimplemented!()
    }
}

impl<I: Iterator> ExtendedIterator for I { }