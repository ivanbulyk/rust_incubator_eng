#![allow(unused)]
use std::io;

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
struct Point {
    x: f32,
    y: f32,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
struct CustomNonEmpty<Point> {
    initial_elem: Point,
    body: Vec<Point>,
}

#[derive(Debug, Clone)]
struct Polyline {
    points: CustomNonEmpty<Point>,
}

impl<Point> CustomNonEmpty<Point> {
    fn new(e: Point) -> Self {
        Self::origin(e)
    }

    /// Create a new non-empty list with an initial element.
    fn origin(initial_elem: Point) -> Self {
        CustomNonEmpty {
            initial_elem,
            body: Vec::new(),
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
