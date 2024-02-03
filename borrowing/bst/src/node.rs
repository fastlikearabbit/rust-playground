#![forbid(unsafe_code)]
pub struct Node<K, V> {
    key        : K,
    data       : V,
    height:    : usize,
    left_child : Option<Box<Node<K, V>>>,
    right_child: Option<Box<Node<K, V>>>,
}

impl<K: Ord, V> Node<K, V> {
    // TODO: your code goes here.
}
