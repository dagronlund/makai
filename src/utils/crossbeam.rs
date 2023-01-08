use std::collections::VecDeque;

use crossbeam::channel::{Receiver, RecvError, SendError, Sender};

pub struct SenderQueued<T> {
    sender: Sender<Vec<T>>,
    queue: Vec<T>,
}

impl<T> SenderQueued<T> {
    pub fn new(sender: Sender<Vec<T>>, limit: usize) -> Self {
        Self {
            sender,
            queue: Vec::with_capacity(limit),
        }
    }

    pub fn send(&mut self, value: T) -> Result<(), SendError<Vec<T>>> {
        self.queue.push(value);
        self.flush()
    }

    pub fn flush(&mut self) -> Result<(), SendError<Vec<T>>> {
        if self.queue.len() >= self.queue.capacity() {
            let mut queue_swap = Vec::with_capacity(self.queue.capacity());
            std::mem::swap(&mut self.queue, &mut queue_swap);
            self.sender.send(queue_swap)
        } else {
            Ok(())
        }
    }

    pub fn finish(self) -> Result<(), SendError<Vec<T>>> {
        if !self.queue.is_empty() {
            self.sender.send(self.queue)?;
            self.sender.send(Vec::new())
        } else {
            self.sender.send(self.queue)
        }
    }
}

pub struct ReceiverQueued<T> {
    receiver: Receiver<Vec<T>>,
    queue: VecDeque<T>,
    done: bool,
}

impl<T> ReceiverQueued<T> {
    pub fn new(receiver: Receiver<Vec<T>>) -> Self {
        Self {
            receiver,
            queue: VecDeque::new(),
            done: false,
        }
    }

    pub fn recv(&mut self) -> Result<Option<T>, RecvError> {
        if self.done {
            return Ok(None);
        }
        if self.queue.is_empty() {
            self.queue = VecDeque::from(self.receiver.recv()?);
            if self.queue.is_empty() {
                self.done = true;
                return Ok(None);
            }
        }
        Ok(self.queue.pop_front())
    }
}
