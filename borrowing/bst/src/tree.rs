#![forbid(unsafe_code)]
use std::{ borrow::Borrow, cmp::Ordering };
use crate::node::Node;

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

    fn fix_height(root: &mut Box<Node<K, V>>) {
        let left_height  = Self::height(&root.left_child);
        let right_height = Self::height(&root.right_child);
        root.height = 1 + left_height.max(right_height);

        let left_size = Self::size(&root.left_child);
        let right_size = Self::size(&root.right_child);
        root.size = 1 + left_size + right_size;
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
            return (Some(node), None);
        }
        let mut root = root.unwrap();
        match node.key.cmp(root.key.borrow()) {
            Ordering::Less => {
                let (updated_left, replaced_value) = Self::insert_recursive(root.left_child.take(), node);
                root.left_child = updated_left;
                (Some(root), replaced_value)
            },
            Ordering::Equal => {
                let previous_value = root.value;
                root.value = node.value;
                (Some(root), Some(previous_value))
            },
            Ordering::Greater => {
                let (updated_right, replaced_value) = Self::insert_recursive(root.right_child.take(), node);
                root.right_child = updated_right;
                (Some(root), replaced_value)
            }
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let (root, previous_value) = 
            Self::insert_recursive(self.root.take(), Node::leaf(key, value).into());
        self.root = root;
        previous_value
    }

    fn remove_entry_recursive<Q>(root: Option<Box<Node<K, V>>>, key: &Q) -> (Option<Box<Node<K, V>>>, Option<(K, V)>)
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        if root.is_none() { return (None, None); }

        let mut root = root.unwrap();
        match key.cmp(root.key.borrow()) {
            Ordering::Less => {
                let (updated_left, removed_entry) = Self::remove_entry_recursive(root.left_child.take(), key);
                root.left_child = updated_left;
                (Some(root), removed_entry)
            }
            Ordering::Equal => {
                match (root.left_child.take(), root.right_child.take()) {
                    (None, None) => (None, Some((root.key, root.value))), // No children

                    (Some(left_child), None) => (Some(left_child), Some((root.key, root.value))), // Only left child

                    (None, Some(right_child)) => (Some(right_child), Some((root.key, root.value))), // Only right child
                    
                    (Some(left_child), Some(right_child)) => {
                        // Two children: Find the in-order successor.
                        let (right_with_successor_removed, mut successor_node) = Self::remove_min(right_child);

                        std::mem::swap(&mut root.key, &mut successor_node.key);
                        std::mem::swap(&mut root.value, &mut successor_node.value);

                        root.left_child = Some(left_child);
                        root.right_child = right_with_successor_removed;

                        (Some(root), Some((successor_node.key, successor_node.value)))
                    }
                }
            },
            Ordering::Greater => {
                let (updated_right, removed_entry) = Self::remove_entry_recursive(root.right_child.take(), key);
                root.right_child = updated_right;
                (Some(root), removed_entry)
            },
        }
    }

    fn remove_min(mut node: Box<Node<K, V>>) -> (Option<Box<Node<K, V>>>, Box<Node<K, V>>) {
        match node.left_child.take() {
            Some(left) => {
                let (new_left, min_node) = Self::remove_min(left);
                node.left_child = new_left;
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
       let (mut root, removed_entry) = Self::remove_entry_recursive(self.root.take(), key);
       self.root = root.take();
       removed_entry
    }
}
