#![forbid(unsafe_code)]
pub struct Node<K, V> {
    pub key        : K,
    pub value      : V,
        height     : usize,
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
                height: 0,
                left_child,
                right_child,
            }
    }
    pub fn leaf(key: K, value: V) -> Self {
        Self {
            key,
            value,
            height: 0,
            left_child: None,
            right_child: None,
        }
    }
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn set_height(&mut self, height: usize) {
        self.height = height;
    }
}
