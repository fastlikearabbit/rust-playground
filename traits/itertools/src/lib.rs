#![forbid(unsafe_code)]

pub struct LazyCycle<I>
where
    I: Iterator,
{
    iter: I,
    saved: Vec<I::Item>,
    saved_index: usize,
}

impl<I> Iterator for LazyCycle<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.saved_index < self.saved.len() {
            let item = self.saved[self.saved_index].clone();
            self.saved_index = (self.saved_index + 1) % self.saved.len();
            Some(item)
        } else {
            match self.iter.next() {
                Some(item) => {
                    self.saved.push(item.clone());
                    self.saved_index += 1; 
                    Some(item)
                }
                None if !self.saved.is_empty() => {
                    self.saved_index = 0;
                    self.next()
                }
                _ => None,
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct Extract<I: Iterator> {
    iter: I,
}

impl<I: Iterator> Iterator for Extract<I> {
    type Item = I::Item;

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
    type Item = I::Item;

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
        LazyCycle {
            iter: self,
            saved: Vec::new(),
            saved_index: 0,
        }
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