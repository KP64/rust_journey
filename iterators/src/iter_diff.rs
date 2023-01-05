#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

pub(crate) fn iter_diff() {
    println!("Iter Diff:");

    // * by value iteration
    let points = vec![
        Point { x: 1.0, y: 1.0 },
        Point { x: 2.0, y: 2.5 },
        Point { x: 4.5, y: 10.0 },
    ];
    let first_point = points.into_iter().next().unwrap();
    println!("{:?}", first_point);

    // * by reference iteration
    let points = vec![
        Point { x: 1.0, y: 1.0 },
        Point { x: 2.0, y: 2.5 },
        Point { x: 4.5, y: 10.0 },
    ];
    // * or: (&points).into_iter()
    let mut iter = points.iter();
    let first_point = iter.next().unwrap();
    println!("{:?}", first_point);

    // * Mut Iter
    let mut points = vec![
        Point { x: 1.0, y: 1.0 },
        Point { x: 2.0, y: 2.5 },
        Point { x: 4.5, y: 10.0 },
    ];
    // * or: (&mut points).into_iter()
    let mut iter = points.iter_mut();
    let first_point = iter.next().unwrap();
    first_point.x = 4.0;
    first_point.y = 100.0;
    println!("{:?}", first_point);
}
