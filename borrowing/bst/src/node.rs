#![forbid(unsafe_code)]

#[derive(Debug)]
pub struct Node<K, V> {
    pub key        : K,
    pub value      : V,
    pub height     : usize,
    pub size       : usize, 
    pub left_child : Option<Box<Node<K, V>>>,
    pub right_child: Option<Box<Node<K, V>>>,
}

impl<K: Ord, V> Node<K, V> {
    pub fn new(key: K, value: V, 
        left_child: Option<Box<Node<K, V>>>, 
        right_child: Option<Box<Node<K, V>>>) -> Self {
            Self {
                key,
                value,
                height: 1,
                size  : 1,
                left_child,
                right_child,
            }
    }
    pub fn leaf(key: K, value: V) -> Self {
        Self {
            key,
            value,
            height: 1,
            size  : 1,
            left_child: None,
            right_child: None,
        }
    }
}


impl<K, V> From<Node<K, V>> for Option<Box<Node<K, V>>> {
    fn from(node: Node<K, V>) -> Self {
        Some(Box::new(node))
    }
}