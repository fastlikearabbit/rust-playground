#![forbid(unsafe_code)]
pub struct Node<K, V> {
    key        : K,
    value      : V,
    height     : usize,
    left_child : Option<Box<Node<K, V>>>,
    right_child: Option<Box<Node<K, V>>>,
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
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn set_height(&mut self, height: usize) {
        self.height = height;
    }
}
