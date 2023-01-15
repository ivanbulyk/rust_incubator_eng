#![allow(unused)]
use core::ops::Deref;
use std::{io, mem};

/*
impl Default for Point {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

impl Clone for Point {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for Point {}
*/
#[derive(PartialOrd, Copy, Clone, Default, PartialEq, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CustomNonEmpty<Point> {
    initial_elem: Point,
    body: Vec<Point>,
}

#[derive(Debug, Clone)]
pub struct Polyline {
    pub points: CustomNonEmpty<Point>,
}

impl<Point> CustomNonEmpty<Point> {
    pub fn new(e: Point) -> Self {
        Self::origin(e)
    }

    /// Create a new non-empty list with an initial element.
    fn origin(initial_elem: Point) -> Self {
        CustomNonEmpty {
            initial_elem,
            body: Vec::new(),
        }
    }

    /// Get the length of the list.
    pub fn len(&self) -> usize {
        self.body.len() + 1
    }

    /// Inserts an element at position index within the vector, shifting all elements after it to the right.

    /// Panics if index > len.

    pub fn insert(&mut self, index: usize, element: Point) {
        let len = self.len();
        assert!(index <= len);

        if index == 0 {
            let initial_elem = mem::replace(&mut self.initial_elem, element);
            self.body.insert(0, initial_elem);
        } else {
            self.body.insert(index - 1, element);
        }
    }

    /// Get an element by index.
    pub fn get(&self, index: usize) -> Option<&Point> {
        if index == 0 {
            Some(&self.initial_elem)
        } else {
            self.body.get(index - 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_default() {
        let a: Point = Default::default();
        let b = Point { x: 0.0, y: 0.0 };
        assert_eq!((a.x, a.y), (b.x, b.y));
    }

    #[test]
    fn point_copy() {
        let a: Point = Default::default();
        let b = a;
        assert_eq!((a.x, a.y), (b.x, b.y));
    }

    #[test]
    fn polyline_clone() {
        let mut v = CustomNonEmpty::new(Point::default());
        let z = v.clone();
        assert_ne!(None, Some(z));
    }
}
