#![forbid(unsafe_code)]

use std::{cell::RefCell, collections::VecDeque, num::NonZeroUsize, rc::Rc};

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
    iter: std::iter::Chain<std::vec::IntoIter<<I as Iterator>::Item>, I>,
}

impl<I: Iterator> Iterator for Extract<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

////////////////////////////////////////////////////////////////////////////////

struct Cache<I: Iterator> {
    iter: I,
    cache: VecDeque<I::Item>,
    id: usize,
    is_iter_over: bool,
}

pub struct Tee<I>
where
    I: Iterator,
    I::Item: Clone,
{
    buffer: Rc<RefCell<Cache<I>>>,
    id: usize,
}

impl<I> Iterator for Tee<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = self.buffer.borrow_mut();
        if buffer.id == self.id {
            match buffer.cache.pop_front() {
                Some(e) => return Some(e),
                None => { },
            }
        }
        if buffer.is_iter_over { return None; }
        match buffer.iter.next() {
            Some(e) => {
                buffer.cache.push_back(e.clone());
                buffer.id = if self.id == 1 { 2 } else { 1 };
                Some(e)
            },
            None => {
                buffer.is_iter_over = true;
                None
            },
        }
    }
}
////////////////////////////////////////////////////////////////////////////////

pub struct GroupBy<I, F, V>
where
    I: Iterator,
    F: FnMut(&I::Item) -> V,
    V: Eq,
{
    iter: std::iter::Peekable<I>,
    f: F,
}

impl<I, F, V> Iterator for GroupBy<I, F, V>
where
    I: Iterator,
    F: FnMut(&I::Item) -> V,
    V: Eq,
{
    type Item = (V, std::vec::IntoIter<I::Item>);

    fn next(&mut self) -> Option<Self::Item> {
        let key = self.iter.next()?;
        let v = (self.f)(&key);
        let mut values = Vec::new();
        values.push(key);
        while let Some(e) = self.iter.peek() {
            if (self.f)(e) == v {
                values.push(self.iter.next().unwrap())
            } else {
                break;
            }
        }
        Some((v, values.into_iter()))
    }
}

////////////////////////////////////////////////////////////////////////////////

pub trait ExtendedIterator: Iterator {
    fn lazy_cycle(self) -> LazyCycle<Self>
    where
        Self: Sized,
        Self::Item: Clone,
    {
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
        let mut index = index;
        let mut front = Vec::new();
        while index > 0 {
            if let Some(e) = self.next() {
                front.push(e);
            }
            index -= 1;
        }

        let result = self.next().take();
        let new_iter = front.into_iter().chain(self);
        (result, Extract { iter: new_iter })
    }

    fn tee(self) -> (Tee<Self>, Tee<Self>)
    where
        Self: Sized,
        Self::Item: Clone,
    {
        let cache = Cache {
            iter: self,
            cache: VecDeque::new(),
            id: 0,
            is_iter_over: false,
        };

        let first = Tee {
            buffer: Rc::new(RefCell::new(cache)),
            id: 1,
        };
        
        let second = Tee {
            buffer: first.buffer.clone(),
            id: 2,
        };
        
        (first, second)
        
    }

    fn group_by<F, V>(self, func: F) -> GroupBy<Self, F, V>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> V,
        V: Eq,
    {
        GroupBy {
            iter: self.peekable(),
            f: func,
        }
    }
}

impl<I: Iterator> ExtendedIterator for I { }