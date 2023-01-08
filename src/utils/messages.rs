use std::any::Any;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct MessagesInternal {
    buffer: Vec<Box<dyn Any + Send + Sync>>,
    activity: bool,
}

#[derive(Clone, Debug)]
pub struct Messages {
    internal: Arc<Mutex<MessagesInternal>>,
}

impl Messages {
    pub fn new() -> Self {
        Self {
            internal: Arc::new(Mutex::new(MessagesInternal {
                buffer: Vec::new(),
                activity: false,
            })),
        }
    }

    pub fn push_any(&self, message: Box<dyn Any + Send + Sync>) {
        let mut internal = self.internal.lock().unwrap();
        internal.activity = true;
        internal.buffer.push(message);
    }

    pub fn append_any(&self, mut messages: Vec<Box<dyn Any + Send + Sync>>) {
        let mut internal = self.internal.lock().unwrap();
        internal.activity = true;
        internal.buffer.append(&mut messages);
    }

    pub fn push<T>(&self, message: T)
    where
        T: 'static + Send + Sync,
    {
        let mut internal = self.internal.lock().unwrap();
        internal.activity = true;
        internal.buffer.push(Box::new(message));
    }

    pub fn append<T>(&self, messages: Vec<T>)
    where
        T: 'static + Send + Sync,
    {
        let mut internal = self.internal.lock().unwrap();
        internal.activity = true;
        for message in messages {
            internal.buffer.push(Box::new(message));
        }
    }

    pub fn get<T>(&self) -> Vec<T>
    where
        T: 'static,
    {
        let mut internal = self.internal.lock().unwrap();
        // Turn the internal buffer into a dequeue
        let mut buffer = Vec::new();
        std::mem::swap(&mut buffer, &mut internal.buffer);
        let mut buffer = VecDeque::from(buffer);
        // Iterate through the elements, seperating matching types and not
        let mut unused = Vec::new();
        let mut result = Vec::new();
        while let Some(message) = buffer.pop_front() {
            match message.downcast::<T>() {
                Ok(message) => {
                    internal.activity = true;
                    result.push(*message);
                }
                Err(message) => unused.push(message),
            }
        }
        internal.buffer = unused;
        result
    }

    pub fn check_activity(&self) -> bool {
        let mut internal = self.internal.lock().unwrap();
        let last = internal.activity;
        internal.activity = false;
        last
    }

    pub fn is_empty(&self) -> bool {
        let internal = self.internal.lock().unwrap();
        internal.buffer.is_empty()
    }
}

impl Default for Messages {
    fn default() -> Self {
        Self::new()
    }
}
