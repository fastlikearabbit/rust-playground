#![forbid(unsafe_code)]
use crate::handle::{DirHandle, FileHandle, Handle};

type Callback<'a> = dyn FnMut(&mut Handle) + 'a;

#[derive(Default)]
pub struct Walker<'a> {
    callbacks: Vec<Box<Callback<'a>>>,
}

impl<'a> Walker<'a> {
    pub fn new() -> Self {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn add_callback<F>(&mut self, callback: F)
    where
        F: FnMut(&mut Handle) + 'a,
    {
        self.callbacks.push(Box::new(callback));
    }

    pub fn walk<P: AsRef<std::path::Path>>(mut self, path: P) -> std::io::Result<()> {
        // TODO: your code goes here.
        unimplemented!()
    }

    // TODO: your code goes here.
}
