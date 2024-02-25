#![forbid(unsafe_code)]

//use core::slice::SlicePattern;
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
        let mut previous_value = None;
        let id = self.0.partition_point(|(k, _)| k < key.borrow());

        if id == self.len() {
            self.0.push((key, value));
        } else if self.0[id].0 == key {
            previous_value = Some(std::mem::replace(&mut self.0[id], (key, value)));
        } else {
            self.0.insert(id, (key, value));
        }

        previous_value.map(|(_, v)| v)
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where 
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        match self.0.binary_search_by_key(&key, |(k, _)| k.borrow()) {
            Ok(id) => Some(&self.0[id].1),
            Err(_) => None,
        }
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where 
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
       self.remove_entry(key).map(|(_, v)| v)
    }

    pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where 
    K: Borrow<Q>,
    Q: Ord + ?Sized,
    {
        match self.0.binary_search_by_key(&key, |(k, _)| k.borrow()) {
            Ok(id) => Some(self.0.remove(id)),
            Err(_) => None,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

impl<K, Q, V> Index<&Q> for FlatMap<K, V>
where
    K: Borrow<Q> + Ord,
    Q: Ord + ?Sized,
{
    type Output = V;

    fn index(&self, index: &Q) -> &Self::Output {
        match self.get(index) {
            Some(value) => value,
            None => panic!("Index out of bounds"),
        }
    }
}

impl<K: Ord, V> Extend<(K, V)> for FlatMap<K, V> {
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        for (key, value) in iter {
            self.insert(key, value);
        }
    }
}

impl<K: Ord, V> From<Vec<(K, V)>> for FlatMap<K, V> {
    fn from(mut value: Vec<(K, V)>) -> Self {
        value.sort_by(|first, second| first.0.cmp(&second.0));
        let mut result: Vec<(K, V)> = Vec::new();
        for (k, v) in value.into_iter().rev() {
            if let Some(key) = result.last() {
                if key.0.cmp(&k) == std::cmp::Ordering::Equal { continue; } 
            }
            result.push((k, v));

        }
        result.reverse();
        Self(result)
    }
}

impl<K: Ord, V> From<FlatMap<K, V>> for Vec<(K, V)> {
    fn from(value: FlatMap<K, V>) -> Self {
        value.0
    }
}

impl<K: Ord, V> FromIterator<(K, V)> for FlatMap<K, V> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut flat_map = FlatMap::new();

        for (key, val) in iter {
            flat_map.insert(key, val);
        }

        flat_map
    }
}

impl<K: Ord, V> IntoIterator for FlatMap<K, V> {
    type Item = (K, V);

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

