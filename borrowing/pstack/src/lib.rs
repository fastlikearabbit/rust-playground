#![forbid(unsafe_code)]
use std::{path::Iter, rc::Rc};

pub struct PRef<T> {
    data: T,
    prev: Rc<PRef<T>>,
}

impl<T> std::ops::Deref for PRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct PStack<T> {
    top : Option<PRef<T>>,
    size: usize,
}

impl<T> Default for PStack<T> {
    fn default() -> Self {
        Self {
            top : None,
            size: 0,
        }
    }
}

impl<T> Iterator for PStack<T> {
    type Item = PRef<T>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!();
    }
}

impl<T> Clone for PStack<T> {
    fn clone(&self) -> Self {
        // TODO: your code goes here.
        unimplemented!()
    }
}

impl<T> PStack<T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push(&self, value: T) -> Self {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn pop(&self) -> Option<(PRef<T>, Self)> {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = PRef<T>> {
        self.clone()
    }
}

