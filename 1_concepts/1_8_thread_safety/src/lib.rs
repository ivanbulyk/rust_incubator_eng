#![allow(unused)]
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

struct OnlySync {
    value: Rc<u32>,
}

unsafe impl Sync for OnlySync {}

impl OnlySync {
    fn new(value: u32) -> Self {
        OnlySync {
            value: Rc::new(value),
        }
    }
}

struct OnlySend {
    value: u32,
}

unsafe impl Send for OnlySend {}

impl OnlySend {
    fn new(value: u32) -> Self {
        OnlySend { value }
    }

    fn spawn(self) {
        thread::spawn(move || {
            println!("OnlySend value: {}", self.value);
        });
    }
}

struct SyncAndSend {
    value: AtomicUsize,
}

unsafe impl Send for SyncAndSend {}
unsafe impl Sync for SyncAndSend {}

impl SyncAndSend {
    fn new() -> Self {
        SyncAndSend {
            value: AtomicUsize::new(0),
        }
    }

    fn increment(&self) {
        self.value.fetch_add(1, Ordering::SeqCst);
    }

    fn get(&self) -> usize {
        self.value.load(Ordering::SeqCst)
    }
}

struct NotSyncNotSend {
    value: Rc<u32>,
}

impl NotSyncNotSend {
    fn new(value: u32) -> Self {
        NotSyncNotSend {
            value: Rc::new(value),
        }
    }
}

mod tests {
    use super::*;
}
