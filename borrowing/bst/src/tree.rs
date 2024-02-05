#![forbid(unsafe_code)]
use std::borrow::Borrow;
use std::hash::Hash;

use crate::node::Node;

pub struct AVLTreeMap<K, V> {
    root: Option<Node<K, V>>,
    len : usize,
}

impl<K: Ord, V> Default for AVLTreeMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Ord, V> AVLTreeMap<K, V> {
    pub fn new() -> Self {
        Self {
            root: None,
            len : 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    fn rotate_left(root: Option<Box<Node<K, V>>>) -> Option<Box<Node<K, V>>> {
        todo!("Implement rotate_left");
    }

    fn rotate_right(root: Option<Box<Node<K, V>>>) -> Option<Box<Node<K, V>>> {
        todo!("Implement rotate_right");
    }

    fn rebalance_left(root: Option<Box<Node<K, V>>>) -> Option<Box<Node<K, V>>> {
        todo!("Implement rebalance_left");
    }

    fn rebalance_right(root: Option<Box<Node<K, V>>>) -> Option<Box<Node<K, V>>> {
        todo!("Implement rebalance");
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: ?Sized, {
        todo!();
    }
    
    pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: ?Sized,  {
        todo!();
    }
    
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: ?Sized {
        todo!();
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        todo!();
    }
    
    pub fn nth_key_value(&self, k: usize) -> Option<(&K, &V)> {
        todo!();
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V> 
    where
        K: Borrow<Q>,
        Q: ?Sized, {
        todo!();
    }
    
    pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: ?Sized, {
        todo!();
    }
}
