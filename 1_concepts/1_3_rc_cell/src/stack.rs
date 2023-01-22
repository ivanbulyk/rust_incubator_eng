use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct GlobalStack<T>(Rc<RefCell<Vec<T>>>);

impl<T> GlobalStack<T> {
    pub fn new() -> Self {
        GlobalStack(Rc::new(RefCell::new(Vec::new())))
    }

    pub fn push(&self, elem: T) {
        let mut val = self.0.borrow_mut();
        val.push(elem)
    }

    pub fn pop(&self) -> Option<T> {
        let mut val = self.0.borrow_mut();
        val.pop()
    }

    pub fn is_empty(&self) -> bool {
        let val = self.0.borrow_mut();
        val.is_empty()
    }
}

impl<T: std::clone::Clone> GlobalStack<T> {
    pub fn peek(&self) -> Option<T> {
        let val = self.0.borrow_mut();

        let x = val.last().as_deref().cloned();
        x
    }
}

impl<T> Clone for GlobalStack<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
