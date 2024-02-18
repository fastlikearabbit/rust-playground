#![forbid(unsafe_code)]
use std::borrow::Borrow;
use crate::node::Node;

pub struct AVLTreeMap<K, V> {
    root: Option<Node<K, V>>,
    height : usize,
}

impl<K: Ord, V> Default for AVLTreeMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Ord, V> AVLTreeMap<K, V> {
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

    fn height(root: &Option<Box<Node<K, V>>>) -> usize {
        if let Some(r) = root {
            r.height
        } else {
            0
        }
    }

    fn fix_height(mut root: Option<&mut Box<Node<K, V>>>) {
        let root = root.unwrap();
        let left_height  = Self::height(&root.left_child);
        let right_height = Self::height(&root.right_child);
        root.height = if left_height > right_height { left_height + 1} else { right_height + 1};
    }
    /// assumes root has right child
    fn rotate_left(mut root: Option<Box<Node<K, V>>>) -> Option<Box<Node<K, V>>> {
        let mut root = root.unwrap();
        
        // takes ownership of root's right child
        let mut right_of_root = root.right_child.take().unwrap();
        
        // root takes ownership of right_of_root's left child
        root.right_child = right_of_root.left_child.take();

        Self::fix_height(Some(&mut root));

        // right_of_root's left child takes ownership of root
        right_of_root.left_child = Some(root);

        Self::fix_height(Some(&mut right_of_root));
        
        Some(right_of_root)
    }

    /// assumes root has right child
    fn rotate_right(root: Option<Box<Node<K, V>>>) -> Option<Box<Node<K, V>>> {
        let mut root = root.unwrap();
        
        // takes ownership of root's left child
        let mut left_of_root = root.left_child.take().unwrap();
        
        // root takes ownership of left_of_root's right child
        root.left_child = left_of_root.right_child.take();

        Self::fix_height(Some(&mut root));

        // right_of_root's right child takes ownership of root
        left_of_root.right_child = Some(root);

        Self::fix_height(Some(&mut left_of_root));
   
        Some(left_of_root)
    }

    fn rebalance_left(mut root: Option<Box<Node<K, V>>>) -> Option<Box<Node<K, V>>> {
        if Self::height(&root.as_ref().unwrap().left_child) - Self::height(&root.as_ref().unwrap().right_child) == 2 {
            Self::rotate_right(root)
        } else {
            let mut root = root.unwrap();
            let mut left_child = root.left_child;
            root.left_child = Self::rotate_left(left_child.take());
            
            Self::rotate_right(Some(root))
        }
    }

    fn rebalance_right(root: Option<Box<Node<K, V>>>) -> Option<Box<Node<K, V>>> {
        if Self::height(&root.as_ref().unwrap().right_child) - Self::height(&root.as_ref().unwrap().left_child) == 2 {
            Self::rotate_left(root)
        } else {
            let mut root = root.unwrap();
            let mut right_child = root.right_child;
            root.right_child = Self::rotate_right(right_child.take());
            
            Self::rotate_left(Some(root))
        }
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
