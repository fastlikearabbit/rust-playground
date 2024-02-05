#![forbid(unsafe_code)]

use std::collections::HashMap;
use std::any::Any;

pub struct Context {
    data: HashMap<String, Box<dyn Any>>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: impl AsRef<str>, obj: impl Any) {
        self.data.insert(key.as_ref().to_string(), Box::new(obj));
    }

    pub fn get<T: 'static + Any>(&self, key: impl AsRef<str>) -> &T {
        match self.data.get(key.as_ref()) {
            Some(obj) => match obj.downcast_ref::<T>() {
                Some(val) => val,
                None => panic!("Object with given type missing.")
            }
            None => panic!("Object with given key missing."),
        }
    }

    pub fn insert_singletone<T: Any>(&mut self, obj: T) {
        todo!();
    }
    pub fn get_singletone<T: Any>(&self) -> &T {
        todo!();
    }
}
