#![forbid(unsafe_code)]

use core::slice::SlicePattern;
use std::{borrow::Borrow, iter::FromIterator, ops::Index};

////////////////////////////////////////////////////////////////////////////////

#[derive(Default, Debug, PartialEq, Eq)]
pub struct FlatMap<K, V>(Vec<(K, V)>);

impl<K: Ord, V> FlatMap<K, V> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    pub fn as_slice(&self) -> &[(K, V)] {
        self.0.as_slice()
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        // TODO: your code goes here.
        unimplemented!()
    }

    // pub fn get(&self, key: ???) -> Option<&V>;

    // pub fn remove(&mut self, key: ???) -> Option<V>;

    // pub fn remove_entry(&mut self, key: ???) -> Option<(K, V)>;
}

////////////////////////////////////////////////////////////////////////////////

// impl Index<???> for FlatMap { ... }

// impl Extend<(K, V)> for FlatMap { ... }

// impl From<Vec<(K, V)>> for FlatMap { ... }

// impl From<FlatMap<K, V>> for Vec<(K, V)> { ... }

// impl FromIterator<(K, V)> for FlatMap { ... }

// impl IntoIterator for FlatMap { ... }

