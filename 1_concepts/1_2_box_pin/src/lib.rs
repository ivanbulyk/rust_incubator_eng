#![allow(unused)]
use std::io::Read;
use std::rc::Rc;
use std::{fmt, pin::Pin};

trait MutMeSomehow {
    fn mut_me_somehow(self: std::pin::Pin<&mut Self>);
}

impl<T: std::clone::Clone> MutMeSomehow for Box<T> {
    fn mut_me_somehow(mut self: std::pin::Pin<&mut Self>) {
      let mut a =  self.clone();
      self.set(a);
    }
}

impl<T: std::fmt::Display> MutMeSomehow for Rc<T> {
    fn mut_me_somehow(mut self: std::pin::Pin<&mut Self>) {
        let mut a = self.clone();
        self.clone_from(&a);
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
        let mut x = self.clone();
        let mut y = self.clone();
        x.clone_from(&y);
        self.set(x);
    }
}

mod t_parent {
    use std::{fmt, io, pin::Pin, ops::Deref};

    trait MutMeSomehow {
        fn mut_me_somehow(self: std::pin::Pin<&mut Self>);
    }


    impl<T: std::clone::Clone> MutMeSomehow for T {

        fn mut_me_somehow(mut self: std::pin::Pin<&mut Self>) {

            let mut d = self.clone();
            self.set(d);
        }
    }

    trait SayHi: fmt::Debug {
        fn say_hi(self: Pin<&Self>) {
            println!("Hi from {:?}", self)
        }
    }

    impl<T: fmt::Debug> SayHi for T {
        fn say_hi(self: Pin<&Self>) {
            println!("Hi from {:?}", self)
        }
    }
}

trait SayHi: fmt::Debug {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self)
    }
}

impl<T: fmt::Debug> SayHi for Box<T> {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self)
    }
}

impl<T: fmt::Debug> SayHi for Rc<T> {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self)
    }
}

impl<T: fmt::Debug> SayHi for Vec<T> {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self)
    }
}

impl SayHi for String {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self)
    }
}

impl SayHi for &[u8] {
    fn say_hi(self: Pin<&Self>) {
        println!("Hi from {:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}