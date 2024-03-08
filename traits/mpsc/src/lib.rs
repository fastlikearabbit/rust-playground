#![forbid(unsafe_code)]

use std::{borrow::Borrow, cell::{Cell, RefCell}, collections::VecDeque, fmt::Debug, rc::Rc};
use thiserror::Error;

////////////////////////////////////////////////////////////////////////////////

pub struct Channel<T> {
    queue: Rc<RefCell<VecDeque<T>>>,
    producers: Cell<usize>,
}

impl<T> Channel<T> {
    pub fn new() -> Self {
        Self {
            queue: Rc::new(RefCell::new(VecDeque::new())),
            producers: Cell::new(0),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.queue.as_ref().borrow().is_empty()
    }

    pub fn is_closed(&self) -> bool {
        self.producers.get() == 0
    }

    pub fn put(&mut self, value: T) {
        self.queue.borrow_mut().push_back(value);
    }

    /// Only called when queue is not empty
    pub fn get(&mut self) -> T {
        self.queue.borrow_mut().pop_front().unwrap_or_else(|| panic!("get() called on empty queue"))
    }

    pub fn add_producer(&self) {
        self.producers.set(self.producers.get() + 1);
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Error, Debug)]
#[error("channel is closed")]
pub struct SendError<T> {
    pub value: T,
}

pub struct Sender<T> {
    task_queue: Channel<T>,
    channel_id: usize,
}

impl<T> Sender<T> {
    pub fn send(&self, value: T) -> Result<(), SendError<T>> {
        if self.task_queue.is_closed() { return Err(SendError {value }); }
        self.task_queue.queue.borrow_mut().push_back(value);
        Ok(())
    }

    pub fn is_closed(&self) -> bool {
        self.task_queue.is_closed()
    }

    pub fn same_channel(&self, other: &Self) -> bool {
        self.channel_id == other.channel_id
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        // TODO: your code goes here.
        unimplemented!()
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Error, Debug)]
pub enum ReceiveError {
    #[error("channel is empty")]
    Empty,
    #[error("channel is closed")]
    Closed,
}

pub struct Receiver<T> {
    task_queue: Channel<T>,
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Result<T, ReceiveError> {
        if self.task_queue.is_closed() { return Err(ReceiveError::Closed); }
        if self.task_queue.is_empty() { return Err(ReceiveError::Empty); }

        Ok(self.task_queue.queue.borrow_mut().pop_front().unwrap())
    }

    pub fn close(&mut self) {
        // TODO: your code goes here.
        unimplemented!()
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        // TODO: your code goes here.
        unimplemented!()
    }
}

////////////////////////////////////////////////////////////////////////////////

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    // TODO: your code goes here.
    unimplemented!()
}
