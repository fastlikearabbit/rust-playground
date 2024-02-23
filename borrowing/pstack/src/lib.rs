#![forbid(unsafe_code)]
use std::rc::Rc;

#[derive(Debug)]
pub struct PRef<T> {
    data: Rc<T>,
    prev: Option<Box<PRef<T>>>,
}

impl<T> std::ops::Deref for PRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct PStack<T> {
    top : Option<Box<PRef<T>>>,
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
      todo!();
    }
}

impl<T> PStack<T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push(&self, value: T) -> Self {
       let new_element = PRef {
            data: Rc::new(value),
            prev: match &self.top {
                Some(top) => Some(*top.to_owned()),
                None => None,
            }
        };
        
        Self {
            top: Some(Box::new(new_element)),
            size: self.size + 1,
        }

    }

    pub fn pop(&self) -> Option<(PRef<T>, Self)> {
        todo!();
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

