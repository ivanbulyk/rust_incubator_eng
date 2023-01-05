#![allow(unused)]
use std::io::Read;
use std::rc::Rc;
use std::{fmt, pin::Pin};

trait MutMeSomehow {
    fn mut_me_somehow(self: std::pin::Pin<&mut Self>);
}


impl<T: std::fmt::Display> MutMeSomehow for Box<T> {
    fn mut_me_somehow(self: std::pin::Pin<&mut Self>) {
        self.get_mut().to_string();
    }
}

impl<T: std::fmt::Display> MutMeSomehow for Rc<T> {
    fn mut_me_somehow(self: std::pin::Pin<&mut Self>) {
        self.get_mut().clone();
    }
}

impl<T: Clone> MutMeSomehow for Vec<T> {
    fn mut_me_somehow(mut self: std::pin::Pin<&mut Self>) {
        let mut v = self.clone();
        let mut v1 = v.clone();
        v.append(&mut v1);
        self.set(v);
    }
}

impl MutMeSomehow for String {
    fn mut_me_somehow(mut self: std::pin::Pin<&mut Self>) {
        let s = self.repeat(5);
        self.set(s);
    }
}

impl MutMeSomehow for &[u8] {
    fn mut_me_somehow(mut self: std::pin::Pin<&mut Self>) {
        self.get_mut().bytes();
    }
}

mod t_parent {
    trait MutMeSomehow {
        fn mut_me_somehow(self: std::pin::Pin<&mut Self>);
    }

    impl<T,K> MutMeSomehow for (T,K ){
        fn mut_me_somehow(mut self: std::pin::Pin<&mut Self>) {
            self.into_ref();
        }
    }
}

trait SayHi: fmt::Debug {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
