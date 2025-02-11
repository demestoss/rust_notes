use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
};

// Flavours:
// - Synchronous channels: Channel where send() blocks. Limited capacity.
//   - Mutex + Condvar + VecDeque
//   - Atomic VecDeque (atomic queue) + thread::park + thread::Thread::notify
// - Asynchronous channels: Channel where send() cannot block. Unbounded.
//   - Mutex + Condvar + VecDeque
//   - Mutex + Condvar + LinkedList
//   - Atomic linked list, linked list of T
//   - Atomic block linked list (crossbeam), linked list of atomic VecDeque<T>
// - Rendevouz channels: Synchronous channels with capacity = 0. Used for thread synchronization.
// - Oneshot channels: Channel where you can send once. Any capacity, in practice, only one call to send()

// async/await

pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        {
            let mut inner = self.shared.inner.lock().unwrap();
            inner.senders += 1;
        }
        Self {
            shared: Arc::clone(&self.shared),
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders -= 1;
        if inner.senders == 0 {
            self.shared.available.notify_one();
        }
    }
}

impl<T> Sender<T> {
    pub fn send(&mut self, v: T) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.queue.push_back(v);
        drop(inner);
        self.shared.available.notify_one();
    }
}

pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
    buffer: VecDeque<T>,
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Option<T> {
        if let Some(t) = self.buffer.pop_front() {
            return Some(t);
        }

        let mut inner = self.shared.inner.lock().unwrap();
        loop {
            match inner.queue.pop_front() {
                Some(t) => {
                    if !inner.queue.is_empty() {
                        // Get all the queue inside _one_ and _only_ receiver
                        // And put empty queue inside the shared inner
                        core::mem::swap(&mut self.buffer, &mut inner.queue);
                    }
                    return Some(t);
                }
                None if inner.senders == 0 => return None,
                None => {
                    inner = self.shared.available.wait(inner).unwrap();
                }
            }
        }
    }
}

impl<T> Iterator for Receiver<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.recv()
    }
}

struct Shared<T> {
    inner: Mutex<Inner<T>>,
    available: Condvar,
}

struct Inner<T> {
    queue: VecDeque<T>,
    senders: usize,
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Inner {
        queue: VecDeque::new(),
        senders: 1,
    };
    let shared = Shared {
        inner: Mutex::new(inner),
        available: Condvar::new(),
    };
    let shared = Arc::new(shared);
    (
        Sender {
            shared: shared.clone(),
        },
        Receiver {
            shared: shared.clone(),
            buffer: VecDeque::new(),
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ping_pong() {
        let (mut tx, mut rx) = channel();
        tx.send(42);
        assert_eq!(rx.recv(), Some(42));
    }

    #[test]
    fn closed_tx() {
        let (tx, mut rx) = channel::<()>();
        drop(tx);
        assert_eq!(rx.recv(), None);
    }

    #[test]
    fn closed_rx() {
        let (mut tx, rx) = channel();
        drop(rx);
        tx.send(42);
    }
}
