#![forbid(unsafe_code)]

use std::collections::VecDeque;
use std::cmp::min;

#[derive(Default)]
pub struct MinQueue<T> {
    in_stack : VecDeque<(T, T)>,
    out_stack: VecDeque<(T, T)>,
    size: usize,
}

impl<T: Clone + Ord> MinQueue<T> {
    pub fn new() -> Self {
        Self {
            in_stack : VecDeque::new(),
            out_stack: VecDeque::new(),
            size: 0,
        }
    }

    pub fn push(&mut self, val: T) {
        let mut minimum = &val;
        if let Some((_, in_min)) = self.in_stack.front() {
            minimum = min(minimum, in_min);
        }
        let minimum = minimum.clone();
        self.in_stack.push_front((val, minimum));
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.out_stack.is_empty() {
            while let Some((in_top, _)) = self.in_stack.pop_front() {
                let mut minimum = &in_top;
                if let Some((_, out_min)) = self.out_stack.front() {
                    minimum = min(&in_top, out_min);
                }
                let minimum = minimum.clone();
                self.out_stack.push_front((in_top, minimum));
            }
        }
        if let Some((top, _)) = self.out_stack.pop_front() {
            self.size -= 1;
            Some(top)
        } else {
            None
        }
    }

    pub fn front(&self) -> Option<&T> {
        match self.out_stack.front() {
            Some((top, _)) => Some(top),
            _              => if let Some((in_stack_back, _)) = self.in_stack.back() {
                                Some(in_stack_back)
                            } else {
                                None
                            }
        }
    }

    pub fn min(&self) -> Option<&T> {
        match (self.in_stack.front(), self.out_stack.front()) {
            (Some((_, in_min)), Some((_, out_min))) => Some(min(in_min, out_min)),
            (Some((_, in_min)), _) => Some(in_min),
            (_, Some((_, out_min))) => Some(out_min),
            _ => None
        }

    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}
