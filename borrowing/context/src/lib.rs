#![forbid(unsafe_code)]

use std::collections::HashMap;
use std::any::Any;

pub struct Context {
    collection: HashMap<String, Box<dyn Any>>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            collection: HashMap::new(),
        }
    }

    pub fn insert<T: Any>(&mut self, key: impl AsRef<str>, obj: T) {
        todo!();
    }
    pub fn get<T>(&self, key: impl AsRef<str>) -> &T {
        todo!();
    }

    pub fn insert_singletone<T: Any>(&mut self, obj: T) {
        todo!();
    }
    pub fn get_singletone<T>(&self) -> &T {
        todo!();
    }
}
