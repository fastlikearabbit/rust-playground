#![forbid(unsafe_code)]
use std::rc::Rc;

#[derive(Debug)]
pub struct PRef<T> {
    data: Rc<T>,
    prev: Option<Rc<PRef<T>>>,
}

impl<T> std::ops::Deref for PRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct PStack<T> {
    top: Option<Rc<PRef<T>>>,
    size: usize,
}

impl<T> Default for PStack<T> {
    fn default() -> Self {
        Self {
            top: None,
            size: 0,
        }
    }
}

impl<T> Clone for PStack<T> {
    fn clone(&self) -> Self {
        Self {
            top: self.top.clone(),
            size: self.size,
        }
    }
}

impl<T> Iterator for PStack<T> {
    type Item = PRef<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let head = self.top.as_mut()?;
        let to_return = PRef {
            data: head.data.clone(),
            prev: None,
        };
        self.top = head.prev.clone();
        self.size -= 1;
       
        Some(to_return)
    }
}

impl<T> PStack<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&self, value: T) -> Self {
        Self {
            top: Some(Rc::new(
                PRef {
                    data: Rc::new(value),
                    prev: self.top.clone(),
                }
            )),
            size: self.size + 1,
        }
    }

    pub fn pop(&self) -> Option<(PRef<T>, Self)> {
        self.top.as_ref().map(|head| 
            (PRef {
                data: head.data.clone(), 
                prev: None,
            },
            Self { 
                top: head.prev.clone(), 
                size: self.size.saturating_sub(1)
            })
        )
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

