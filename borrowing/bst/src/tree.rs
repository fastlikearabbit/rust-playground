#![forbid(unsafe_code)]
use std::{ borrow::Borrow, cmp::Ordering };
use crate::node::Node;

#[derive(Debug)]
pub struct AVLTreeMap<K, V> {
    root: Option<Box<Node<K, V>>>,
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

    fn height(node: &Option<Box<Node<K, V>>>) -> usize {
        node.as_ref().map_or(0, |n| n.height)

    }

    fn size(node: &Option<Box<Node<K, V>>>) -> usize {
        node.as_ref().map_or(0, |n| n.size)
    }

    fn fix_tree_props(root: &mut Box<Node<K, V>>) {
        // fix height
        let left_height  = Self::height(&root.left_child);
        let right_height = Self::height(&root.right_child);
        root.height = 1 + left_height.max(right_height);

        // fix size
        let left_size = Self::size(&root.left_child);
        let right_size = Self::size(&root.right_child);
        root.size = 1 + left_size + right_size;
    }

    /// assumes root has right child
    fn rotate_left(mut root: Box<Node<K, V>>) -> Box<Node<K, V>> {
        // takes ownership of root's right child
        assert!(root.right_child.is_some());
        let mut right_of_root = root.right_child.take().unwrap();
        
        // root takes ownership of right_of_root's left child
        root.right_child = right_of_root.left_child.take();

        // right_of_root's left child takes ownership of root
        right_of_root.left_child = Some(root);

        // TODO: possible error in how fix_tree_props is called
        Self::fix_tree_props(right_of_root.left_child.as_mut().unwrap());
        Self::fix_tree_props(&mut right_of_root);
        
        right_of_root
    }

    /// assumes root has left child
    fn rotate_right(mut root: Box<Node<K, V>>) -> Box<Node<K, V>> {   
        // takes ownership of root's left child
        assert!(root.left_child.is_some());
        let mut left_of_root = root.left_child.take().unwrap();
        
        // root takes ownership of left_of_root's right child
        root.left_child = left_of_root.right_child.take();

        // right_of_root's right child takes ownership of root
        left_of_root.right_child = Some(root);

        Self::fix_tree_props(left_of_root.right_child.as_mut().unwrap());
        Self::fix_tree_props(&mut left_of_root);
   
        left_of_root
    }

    fn rebalance_left(mut root: Box<Node<K, V>>) -> Box<Node<K, V>> {
        if Self::height(&root.left_child) == Self::height(&root.right_child) + 2 {
            let left_of_root = root.left_child.as_ref().unwrap();
            if Self::height(&left_of_root.left_child) > Self::height(&left_of_root.right_child) {
                root = Self::rotate_right(root);
            } else {
                println!("parents: {} {}", Self::height(&root.right_child), Self::height(&root.left_child));
                println!("children: {} {}", Self::height(&left_of_root.right_child), Self::height(&left_of_root.left_child));
                root.left_child = Some(Self::rotate_left(root.left_child.take().unwrap()));
                root = Self::rotate_right(root);
            }
        } else {
            Self::fix_tree_props(&mut root);
        }

        root
    }

    fn rebalance_right(mut root: Box<Node<K, V>>) -> Box<Node<K, V>> {
        if Self::height(&root.right_child) == Self::height(&root.left_child) + 2 {
            let right_of_root = root.right_child.as_ref().unwrap();
            if Self::height(&right_of_root.right_child) > Self::height(&right_of_root.left_child) {
                root = Self::rotate_left(root);
            } else {

                root.right_child = Some(Self::rotate_right(root.right_child.take().unwrap())); 
                root = Self::rotate_left(root)
            } 
        } else {
            Self::fix_tree_props(&mut root);
        }
        root
    }
    
    pub fn nth_key_value(&self, n: usize) -> Option<(&K, &V)> {
        Self::nth_element_recursive(&self.root, n)
    }

    fn nth_element_recursive(root: &Option<Box<Node<K, V>>>, n: usize) -> Option<(&K, &V)> {
        let left_size = Self::size(&root.as_ref()?.left_child);
        match root {
            Some(ref node) if n < left_size => Self::nth_element_recursive(&node.left_child, n),
            Some(ref node) if n == left_size => Some((&node.key, &node.value)),
            Some(ref node) => Self::nth_element_recursive(&node.right_child, n - left_size - 1),
            _ => None,
        }
    }
    
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized, 
    {
        self.get_key_value(key).map(|(_, v)| v)
    }
    
    fn get_key_value_recursive<'node, Q>(root: &'node Option<Box<Node<K, V>>>, key: &Q) -> Option<(&'node K, &'node V)>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized
    {
        if root.is_none() { return None; }

        let root = root.as_ref().unwrap();
        match key.cmp(root.key.borrow()) {
            Ordering::Less => Self::get_key_value_recursive(&root.left_child, key),
            Ordering::Equal => Some((&root.key, &root.value)),
            Ordering::Greater => Self::get_key_value_recursive(&root.right_child, key),
        }
    }
    pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,  
    {
        Self::get_key_value_recursive(&self.root, key)
    }
    
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized
    {
        self.get(key).is_some()
    }

    fn insert_recursive(root: Option<Box<Node<K, V>>>, node: Box<Node<K, V>>) -> (Option<Box<Node<K, V>>>, Option<V>) {
        if root.is_none() {
            return (Some(Node::leaf(node.key, node.value).into()), None);
        }
        let mut root = root.unwrap();
        match node.key.cmp(root.key.borrow()) {
            Ordering::Less => {
                let (updated_left, replaced_value) = 
                    Self::insert_recursive(root.left_child.take(), node);
                root.left_child = updated_left;
                root = Self::rebalance_left(root);
                (Some(root), replaced_value)
            },
            Ordering::Equal => {
                let previous_value = root.value;
                root.value = node.value;
                (Some(root), Some(previous_value))
            },
            Ordering::Greater => {
                let (updated_right, replaced_value) = 
                    Self::insert_recursive(root.right_child.take(), node);
                root.right_child = updated_right;
                root = Self::rebalance_right(root);
                (Some(root), replaced_value)
            }
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let (root, previous_value) = 
            Self::insert_recursive(self.root.take(), Node::leaf(key, value).into());
        self.root = root;
        if previous_value.is_none() { self.len += 1; }
        previous_value
    }

    // TODO: remove has bug related to rebalancing (call unwrap on None)
    fn remove_entry_recursive<Q>(root: Option<Box<Node<K, V>>>, key: &Q) -> (Option<Box<Node<K, V>>>, Option<(K, V)>)
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        if root.is_none() { return (None, None); }

        let mut root = root.unwrap();
        match key.cmp(root.key.borrow()) {
            Ordering::Less => {
                let (updated_left, removed_entry) = 
                    Self::remove_entry_recursive(root.left_child.take(), key);
                root.left_child = updated_left;
                root = Self::rebalance_right(root);
                (Some(root), removed_entry)
            }
            Ordering::Equal => {
                match (root.left_child.take(), root.right_child.take()) {
                    (None, None) => {
                        (None, Some((root.key, root.value)))
                    }, // No children

                    (Some(mut left_child), None) => {
                        left_child = Self::rebalance_left(left_child);
                        (Some(left_child), Some((root.key, root.value)))
                    }, // Only left child

                    (None, Some(mut right_child)) => {
                        right_child = Self::rebalance_right(right_child);
                        (Some(right_child), Some((root.key, root.value)))
                    }, // Only right child
                    
                    (Some(left_child), Some(right_child)) => {
                        // Two children: Find the in-order successor.
                        let (right_with_successor_removed, mut successor_node) = 
                            Self::remove_min(right_child);
  
                        std::mem::swap(&mut root.value, &mut successor_node.value);
                        std::mem::swap(&mut root.key, &mut successor_node.key);
                        root.left_child = Some(left_child);
                        root.right_child = right_with_successor_removed;
                        root = Self::rebalance_left(root);
                        (Some(root), Some((successor_node.key, successor_node.value)))
                    }
                }
            },
            Ordering::Greater => {
                let (updated_right, removed_entry) = 
                    Self::remove_entry_recursive(root.right_child.take(), key);
                root.right_child = updated_right;
                root = Self::rebalance_left(root);
                (Some(root), removed_entry)
            },
        }
    }

    fn remove_min(mut node: Box<Node<K, V>>) -> (Option<Box<Node<K, V>>>, Box<Node<K, V>>) {
        match node.left_child.take() {
            Some(left) => {
                let (new_left, min_node) = Self::remove_min(left);
                node.left_child = new_left;
                Self::fix_tree_props(&mut node);
                (Some(node), min_node)
            },
            None => {
                // No left child means this node is the minimum.
                // Return the right child to replace this node in the tree,
                // and return this node as the removed minimum node.
                (node.right_child.take(), node)
            }
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
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized, 
    {   
        let (mut root, removed_entry) = 
            Self::remove_entry_recursive(self.root.take(), key);
        self.root = root.take();
        if removed_entry.is_some() { self.len -= 1; }
        removed_entry
    }
}
