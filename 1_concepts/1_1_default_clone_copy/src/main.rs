use step_1_1::{CustomNonEmpty, Point, Polyline};

fn main() {
    println!("Already implemented you!");

    let mut p = Polyline {
        points: CustomNonEmpty::new(Point::default()),
    };

    p.points.insert(0, Point { x: 1.0, y: 1.0 });

    println!("{:#?}", p.points.get(0));
}
