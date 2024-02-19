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
        if self.top.is_none() {
            None
        } else {
            let top = self.top.take();
            self.size -= 1;
            top
        }
    }
}

impl<T> Clone for PStack<T> {
    fn clone(&self) -> Self {
        let mut new_stack = Self::new();
        while let Some(top) = self.top {
            
        }


        Self {
            top: None,
            size: 0,
        }
    }
}

impl<T> PStack<T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push(&self, value: T) -> Self {
        let new_value = Rc::new(value);

        Self::new()
    }

    pub fn pop(&self) -> Option<(PRef<T>, Self)> {
        if self.is_empty() {
            None
        } else {
            
            None
        }

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

