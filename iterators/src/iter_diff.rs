#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

pub(crate) fn iter_diff() {
    println!("Iter Diff:");
    // * by reference iteration
    let mut points = vec![
        Point::new(1.0, 1.0),
        Point::new(2.0, 2.5),
        Point::new(4.5, 10.0),
    ];
    // * or: (&points).into_iter()
    let first_point = points.first().unwrap();
    println!("{first_point:?}");

    // * by &mutable iteration
    // * or: (&mut points).into_iter()
    let first_point = points.iter_mut().next().unwrap();
    first_point.x = 4.0;
    first_point.y = 100.0;
    println!("{first_point:?}");

    // * by value iteration
    let first_point = points.into_iter().next().unwrap();
    println!("{first_point:?}");
}
