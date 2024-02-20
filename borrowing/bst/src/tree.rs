#![forbid(unsafe_code)]
use std::{borrow::Borrow, cmp::Ordering};
use crate::node::Node;

pub struct AVLTreeMap<K, V> {
    root: Option<Box<Node<K, V>>>,
    height : usize,
}

impl<K: Ord, V: Clone> Default for AVLTreeMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Ord, V: Clone> AVLTreeMap<K, V> {
    pub fn new() -> Self {
        Self {
            root   : None,
            height : 0,
        }
    }

    pub fn len(&self) -> usize {
        self.height
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    fn fix_height(mut root: &mut Box<Node<K, V>>) {
        let left_height  = root.left_child.as_ref().unwrap().height();
        let right_height = root.right_child.as_ref().unwrap().height();
        root.set_height(if left_height > right_height { left_height + 1} else { right_height + 1});
    }
    /// assumes root has right child
    fn rotate_left(mut root: Box<Node<K, V>>) -> Box<Node<K, V>> {
        // takes ownership of root's right child
        let mut right_of_root = root.right_child.take().unwrap();
        
        // root takes ownership of right_of_root's left child
        root.right_child = right_of_root.left_child.take();

        Self::fix_height(&mut root);

        // right_of_root's left child takes ownership of root
        right_of_root.left_child = Some(root);

        Self::fix_height(&mut right_of_root);
        
        right_of_root
    }

    /// assumes root has right child
    fn rotate_right(mut root: Box<Node<K, V>>) -> Box<Node<K, V>> {        
        // takes ownership of root's left child
        let mut left_of_root = root.left_child.take().unwrap();
        
        // root takes ownership of left_of_root's right child
        root.left_child = left_of_root.right_child.take();

        Self::fix_height(&mut root);

        // right_of_root's right child takes ownership of root
        left_of_root.right_child = Some(root);

        Self::fix_height(&mut left_of_root);
   
        left_of_root
    }

    fn rebalance_left(mut root: Box<Node<K, V>>) -> Box<Node<K, V>> {
        if root.left_child.as_ref().unwrap().height() - root.right_child.as_ref().unwrap().height() == 2 {
            Self::rotate_right(root)
        } else {
            let mut left_child = root.left_child;
            root.left_child = Some(Self::rotate_left(left_child.unwrap()));
            
            Self::rotate_right(root)
        }
    }

    fn rebalance_right(mut root: Box<Node<K, V>>) -> Box<Node<K, V>> {
        if root.right_child.as_ref().unwrap().height() - root.left_child.as_ref().unwrap().height() == 2 {
            Self::rotate_left(root)
        } else {
            let mut right_child = root.right_child;
            root.right_child = Some(Self::rotate_right(right_child.unwrap()));
            
            Self::rotate_left(root)
        }
    }

    fn get_key_value_helper<'node, Q>(root: &'node Option<Box<Node<K, V>>>, key: &Q) -> Option<(&'node K, &'node V)>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        if root.is_none() { return None; }

        match key.cmp(root.as_ref().unwrap().key.borrow()) {
            Ordering::Less => Self::get_key_value_helper(&root.as_ref().unwrap().right_child, key),
            Ordering::Equal => Some((&root.as_ref()?.key, &root.as_ref()?.value)),
            Ordering::Greater => Self::get_key_value_helper(&root.as_ref().unwrap().left_child, key),
        }
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized, 
    {
        match Self::get_key_value_helper(&self.root, key) {
            Some((_, value)) => Some(value),
            _ => None,
        }
        
    }
    
    pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,  
    {
            Self::get_key_value_helper(&self.root, key)
    }
    
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized
    {
        self.get(key).is_some()
    }

    fn insert_helper(mut root: Option<Box<Node<K, V>>>, key: K, value: V) -> (Box<Node<K, V>>, Option<V>) {
        if root.is_none() {
            root = Node::leaf(key, value).into();
            return (root.unwrap(), None);
        }

        let mut root = root.unwrap();
        let mut result = None;
        match key.cmp(&root.key) {
            Ordering::Less => {
                let (child_root, child_res) = Self::insert_helper(root.left_child, key, value);
                root.left_child = Some(child_root);
                root = Self::rebalance_left(root);
                
            },
            Ordering::Equal => {
                let previous_value = root.value.clone();
                root.value = value;

                result = Some(previous_value);
            },
            Ordering::Greater => { 
                let (child_root, child_res) = Self::insert_helper(root.right_child, key, value);
                root.right_child = Some(child_root);
                root = Self::rebalance_right(root);
            },
        }

        (root, result)
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let root = self.root.take();
        let (newroot, result) = Self::insert_helper(root, key, value);
        self.root = Some(newroot);
        result
    }
    
    pub fn nth_key_value(&self, k: usize) -> Option<(&K, &V)> {
        todo!();
    }

    fn remove_helper<Q>(root: Option<Box<Node<K, V>>>, key: &Q) -> (Option<Box<Node<K, V>>>, Option<V>)
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized, 
    {
        if root.is_none() { return (None, None); }

        (None, None)
    }

    fn find_successor(root: &Box<Node<K, V>>) -> Option<V> {
        let mut current = root;
        while let Some(ref left_child) = current.left_child {
            current = left_child;
        }
        Some(current.value.clone())
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
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized, 
    {   
       todo!();
    }
}
