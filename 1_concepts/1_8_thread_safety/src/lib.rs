#![allow(unused)]
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::thread;

struct OnlySync {
    value: Mutex<u32>,
}

impl OnlySync {
    fn new(value: u32) -> Self {
        OnlySync {
            value: Mutex::new(value),
        }
    }

    fn get_value(&self) -> std::sync::MutexGuard<'_, u32> {
        self.value.lock().unwrap()
    }
}

struct OnlySend {
    value: u32,
}

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
