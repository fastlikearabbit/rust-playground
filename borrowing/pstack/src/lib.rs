#![forbid(unsafe_code)]
use std::rc::Rc;

pub struct PRef<T> {
    data: T,
    prev: Option<Rc<PRef<T>>>,
}

impl<T> std::ops::Deref for PRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct PStack<T> {
    top : Option<Rc<PRef<T>>>,
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
       todo!()
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

impl<T> PStack<T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push(&self, value: T) -> Self {
        let new_ref = Rc::new(PRef {
            data: value,
            prev: self.top.clone(),
        });

        PStack {
            top: Some(new_ref),
            size: self.size + 1,
        }
    }

    pub fn pop(&self) -> Option<(PRef<T>, Self)> {
        self.top.as_ref().and_then(|top_ref| {
            let prev_top = &top_ref.prev;
            let new_stack = PStack {
                top: prev_top.clone(),
                size: self.size.saturating_sub(1),
            };

            Rc::try_unwrap(top_ref.clone()).ok().map(|popped_ref| {
                (popped_ref, new_stack)
            })
        })
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

