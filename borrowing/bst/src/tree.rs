#![forbid(unsafe_code)]
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

    // fn get(&self, key: ...) -> Option<&V>
    // fn get_key_value(&self, key: ...) -> Option<&V>
    // fn contains_key(&self, key: ...) -> bool
    // fn insert(&mut self, key: K, value: V) -> Option<V>
    // fn nth_key_value(&self, k: usize) -> Option<(&K, &V)>
    // fn remove(&mut self, key: ...) -> Option<V>
    // fn remove_entry(&mut self, key: ...) -> Option<(K, V)>
}
