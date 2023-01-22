#![allow(unused)]
use step_1_1::{CustomNonEmpty, Point, Polyline};

fn main() {
    println!("Already implemented you!");

    let mut p = Polyline {
        points: CustomNonEmpty::new(Point::default()),
    };

    let res = p
        .points
        .insert(1, Point { x: 1.0, y: 1.0 })
        .unwrap_or_else(|err| {
            eprintln!("Problem inserting element: {err}");
        });

    println!("{:#?}", p.points.get(0));
}
