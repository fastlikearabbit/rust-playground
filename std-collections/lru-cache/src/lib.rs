#![forbid(unsafe_code)]

use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

#[derive(Debug)]
pub struct LRUCache<K, V> {
    cache: HashMap<K, (V, usize)>,
    lru_map: BTreeMap<usize, K>,
    rank: usize,
    size: usize,
    capacity: usize,
}

impl<K: Clone + Hash + Ord, V> LRUCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0);
        // TODO: Do some prepocessing to make it work faster? 
        Self {
            cache: HashMap::with_capacity(capacity),
            lru_map: BTreeMap::new(),
            rank: 0,
            size: 0,
            capacity,
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        let element = self.cache.remove_entry(key);
        if element.is_none() { return None; }
        let (_, (value, rank)) = element.unwrap();

        let (_, key) = self.lru_map.remove_entry(&rank).unwrap();
        self.lru_map.insert(self.rank + 1, key.clone());
        self.cache.insert(key.clone(), (value, self.rank + 1));

        self.size += 1;
        self.rank += 1;

        match self.cache.get(&key) {
            Some((value, _)) => Some(value),
            None => None,
        }
    
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.cache.contains_key(&key) {
            true => {
                let (old_value, rank) = self.cache.insert(key.clone(), (value, self.rank + 1)).unwrap();
                let (_, key) = self.lru_map.remove_entry(&rank).unwrap();
                self.lru_map.insert(self.rank + 1, key);

                self.rank += 1;
                Some(old_value)
            },
            false => {
                if self.size == self.capacity {
                    let (_, lru_key) = self.lru_map.pop_first().unwrap();
                    let (_, (lru_value, _)) = self.cache.remove_entry(&lru_key).unwrap();

                    self.cache.insert(key.clone(), (value, self.rank + 1));
                    self.lru_map.insert(self.rank + 1, key);
                    
                    self.rank += 1;

                    Some(lru_value)

                } else {
                    self.cache.insert(key.clone(), (value, self.rank + 1));
                    self.lru_map.insert(self.rank + 1, key.clone());
                    
                    self.size += 1;
                    None
                }
            },
        }
    }
}
