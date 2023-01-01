#![allow(unused)]
use nonempty::NonEmpty;
use std::io;

#[derive(PartialEq, Debug)]
struct Point {
    x: f32,
    y: f32,
}

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

#[derive(Debug, Clone)]
struct Polyline {
    points: NonEmpty<Point>,
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
        let v: NonEmpty<Point> = NonEmpty::new(Point {
            ..Default::default()
        });
        let z = v.clone();
        assert_ne!(None, Some(z));
    }
}
