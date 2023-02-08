#![allow(unused)]
use std::sync::{Arc, RwLock};

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Arc<RwLock<Node<T>>>>,
    prev: Option<Arc<RwLock<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node {
            data,
            next: None,
            prev: None,
        }
    }
}

struct DoublyLinkedList<T> {
    head: Option<Arc<RwLock<Node<T>>>>,
    tail: Option<Arc<RwLock<Node<T>>>>,
}

impl<T: Clone + Copy> DoublyLinkedList<T> {
    fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
        }
    }

    fn insert_head(&mut self, data: T) {
        let new_node = Arc::new(RwLock::new(Node::new(data)));
        match self.head.take() {
            None => {
                self.tail = Some(new_node.clone());
                self.head = Some(new_node);
            }
            Some(head) => {
                head.write().unwrap().prev = Some(new_node.clone());
                new_node.write().unwrap().next = Some(head);
                self.head = Some(new_node);
            }
        }
    }

    fn insert_tail(&mut self, data: T) {
        let new_node = Arc::new(RwLock::new(Node::new(data)));
        match self.tail.take() {
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
            Some(tail) => {
                tail.write().unwrap().next = Some(new_node.clone());
                new_node.write().unwrap().prev = Some(tail);
                self.tail = Some(new_node);
            }
        }
    }

    fn remove_head(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            let mut head = head.write().unwrap();
            let data = head.data;
            match head.next.take() {
                None => {
                    self.tail.take();
                }
                Some(next) => {
                    next.write().unwrap().prev.take();
                    self.head = Some(next);
                }
            }
            data
        })
    }

    fn remove_tail(&mut self) -> Option<T> {
        self.tail.take().map(|tail| {
            let mut tail = tail.write().unwrap();
            let data = tail.data;
            match tail.prev.take() {
                None => {
                    self.head.take();
                }
                Some(prev) => {
                    prev.write().unwrap().next.take();
                    self.tail = Some(prev);
                }
            }
            data
        })
    }

    fn peek_front(&self) -> Option<T> {
        self.head.as_ref().map(|head| head.read().unwrap().data)
    }

    fn peek_back(&self) -> Option<T> {
        self.tail.as_ref().map(|tail| tail.read().unwrap().data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_single_threaded() {
        let mut dll = DoublyLinkedList::<i32>::new();

        // insert_head
        dll.insert_head(1);
        assert_eq!(dll.peek_front(), Some(1));
        assert_eq!(dll.peek_back(), Some(1));

        dll.insert_head(2);
        assert_eq!(dll.peek_front(), Some(2));
        assert_eq!(dll.peek_back(), Some(1));

        // insert_tail
        dll.insert_tail(3);
        assert_eq!(dll.peek_front(), Some(2));
        assert_eq!(dll.peek_back(), Some(3));

        dll.insert_tail(4);
        assert_eq!(dll.peek_front(), Some(2));
        assert_eq!(dll.peek_back(), Some(4));

        // remove_head
        assert_eq!(dll.remove_head(), Some(2));
        assert_eq!(dll.peek_front(), Some(1));
        assert_eq!(dll.peek_back(), Some(4));

        assert_eq!(dll.remove_head(), Some(1));
        assert_eq!(dll.peek_front(), Some(3));
        assert_eq!(dll.peek_back(), Some(4));

        // remove_tail
        assert_eq!(dll.remove_tail(), Some(4));
        assert_eq!(dll.peek_front(), Some(3));
        assert_eq!(dll.peek_back(), Some(3));

        assert_eq!(dll.remove_tail(), Some(3));
        assert_eq!(dll.peek_front(), None);
        assert_eq!(dll.peek_back(), None);
    }

    #[cfg(test)]
    #[test]
    fn test_multi_threaded_remove_head_and_peek_front() {
        let dll = Arc::new(RwLock::new(DoublyLinkedList::<i32>::new()));

        let dll_clone = dll.clone();
        let insert_head_handle = thread::spawn(move || {
            let mut dll = dll_clone.write().unwrap();
            for i in 1..=100 {
                dll.insert_head(i);
            }
        });

        insert_head_handle.join().unwrap();

        let dll_clone = dll.clone();
        let remove_head_handle = thread::spawn(move || {
            let mut dll = dll_clone.write().unwrap();
            for _ in 1..=50 {
                dll.remove_head();
            }
        });

        let dll_clone = dll.clone();
        let peek_front_handle = thread::spawn(move || {
            let dll = dll_clone.read().unwrap();
            let data = dll.peek_front();
            assert_eq!(data, Some(50));
        });

        remove_head_handle.join().unwrap();
        peek_front_handle.join().unwrap();
    }

    #[test]
    fn test_multi_threaded_remove_tail_and_peek_back() {
        let dll = Arc::new(RwLock::new(DoublyLinkedList::<i32>::new()));

        let dll_clone = dll.clone();
        let insert_tail_handle = thread::spawn(move || {
            let mut dll = dll_clone.write().unwrap();
            for i in 1..=100 {
                dll.insert_tail(i);
            }
        });

        insert_tail_handle.join().unwrap();

        let dll_clone = dll.clone();
        let remove_tail_handle = thread::spawn(move || {
            let mut dll = dll_clone.write().unwrap();
            for _ in 1..=50 {
                dll.remove_tail();
            }
        });

        let dll_clone = dll.clone();
        let peek_back_handle = thread::spawn(move || {
            let dll = dll_clone.read().unwrap();
            let data = dll.peek_back();
            assert_eq!(data, Some(50));
        });

        remove_tail_handle.join().unwrap();
        peek_back_handle.join().unwrap();
    }
}
