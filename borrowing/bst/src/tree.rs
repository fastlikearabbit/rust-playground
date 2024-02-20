#![forbid(unsafe_code)]
use std::{borrow::Borrow, cmp::Ordering};
use crate::node::Node;

pub struct AVLTreeMap<K, V> {
    root: Option<Box<Node<K, V>>>,
    len : usize,
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
            len : 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    fn height(node: &Option<Box<Node<K, V>>>) -> usize {
        node.as_ref().map_or(0, |n| n.height)
    }

    fn size(node: &Option<Box<Node<K, V>>>) -> usize {
        node.as_ref().map_or(0, |n| n.size)
    }

    fn fix_height(root: &mut Box<Node<K, V>>) {
        let left_height  = Self::height(&root.left_child);
        let right_height = Self::height(&root.right_child);
        root.height = 1 + left_height.max(right_height);

        let left_size = Self::size(&root.left_child);
        let right_size = Self::size(&root.right_child);
        root.size = 1 + left_size + right_size;
    }
    /// assumes root has right child
    fn rotate_left(mut root: Box<Node<K, V>>) -> Box<Node<K, V>> {
        // takes ownership of root's right child
        let mut right_of_root = root.right_child.take().unwrap();
        
        // root takes ownership of right_of_root's left child
        root.right_child = right_of_root.left_child.take();

        // right_of_root's left child takes ownership of root
        right_of_root.left_child = Some(root);

        Self::fix_height(right_of_root.left_child.as_mut().unwrap());
        Self::fix_height(&mut right_of_root);
        
        right_of_root
    }

    /// assumes root has left child
    fn rotate_right(mut root: Box<Node<K, V>>) -> Box<Node<K, V>> {        
        // takes ownership of root's left child
        let mut left_of_root = root.left_child.take().unwrap();
        
        // root takes ownership of left_of_root's right child
        root.left_child = left_of_root.right_child.take();

        // right_of_root's right child takes ownership of root
        left_of_root.right_child = Some(root);

        Self::fix_height(left_of_root.right_child.as_mut().unwrap());
        Self::fix_height(&mut left_of_root);
   
        left_of_root
    }

    fn rebalance_left(mut root: Box<Node<K, V>>) -> Box<Node<K, V>> {
        if Self::height(&root.left_child) == Self::height(&root.right_child) + 2 {
            Self::rotate_right(root)
        } else {
            let mut left_child = root.left_child;
            root.left_child = Some(Self::rotate_left(left_child.unwrap()));
            
            Self::rotate_right(root)
        }
    }

    fn rebalance_right(mut root: Box<Node<K, V>>) -> Box<Node<K, V>> {
        if Self::height(&root.right_child) == Self::height(&root.left_child) + 2 {
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
        self.len += 1;
        result
    }
    
    pub fn nth_key_value(&self, n: usize) -> Option<(&K, &V)> {
        Self::nth_element_helper(&self.root, n)
    }

    fn nth_element_helper(node: &Option<Box<Node<K, V>>>, n: usize) -> Option<(&K, &V)> {
        let left_size = node.as_ref().map_or(0, |n| Self::size(&n.left_child));
        match node {
            Some(ref node) if n <= left_size => Self::nth_element_helper(&node.left_child, n),
            Some(ref node) if n == left_size + 1 => Some((&node.key, &node.value)),
            Some(ref node) => Self::nth_element_helper(&node.right_child, n - left_size - 1),
            _ => None,
        }
    }

    fn take_min_node(mut root: Box<Node<K, V>>) -> (Box<Node<K, V>>, (K, V)) {
        match root.left_child.take() {
            Some(left) => {
                let (new_left, min_key_val) = Self::take_min_node(left);
                root.left_child = Some(new_left);
                (root, min_key_val)
            }
            None => (root.right_child.unwrap(), (root.key, root.value))
        }
    }

    fn remove_helper<Q>(root: Option<Box<Node<K, V>>>, key: &Q) -> (Option<Box<Node<K, V>>>, Option<(K, V)>)
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized, 
    {
        root.map(|mut node|
        {
            match key.cmp(node.key.borrow()) {
                Ordering::Less => {
                    let (new_left, removed_key_val) = Self::remove_helper(node.left_child.take(), key);
                    node.left_child = new_left;
                    node = Self::rebalance_left(node);
                    (Some(node), removed_key_val)
                }
                Ordering::Equal => {
                    let removed_key_val = Some((node.key, node.value));
                    if node.left_child.is_none() {
                        return (node.right_child, removed_key_val);
                    }
                    if node.right_child.is_none() {
                        return (node.left_child, removed_key_val);
                    }

                    let (mut min_node, _) = Self::take_min_node(node.right_child.unwrap());
                    min_node.left_child = node.left_child;
                    Self::fix_height(&mut min_node);
                    (Some(min_node), removed_key_val)
                },
                Ordering::Greater => {
                    let (new_right, removed_key_val) = Self::remove_helper(node.right_child.take(), key);
                    node.right_child = new_right;
                    node = Self::rebalance_right(node);
                    (Some(node), removed_key_val)
                },
            }
        }).unwrap_or((None, None))
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
        let (new_root, removed_key_val) = Self::remove_helper(self.root.take(), key);
        self.root = new_root;
        if removed_key_val.is_some() { self.len -= 1; }
        removed_key_val
    }
}
