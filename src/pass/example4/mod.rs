// taken from Rust By Example

struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32,
}

fn example4() {
    let point: Point = Point { x: 5.2, y: 0.4 };
    let another_point: Point = Point { x: 10.3, y: 0.2 };

    assert!(point.x == 5.2);
    assert!(point.y == 0.4);

    let bottom_right = Point { x: 10.3, ..another_point };

    assert!(bottom_right.x == 10.3);
    assert!(bottom_right.y == 0.2);

    let Point { x: left_edge, y: top_edge } = point;

    assert!(left_edge == 5.2);
    assert!(top_edge == 0.4);

    let pair = Pair(1, 0.1);

    assert!(pair.0 == 1);
    assert!(pair.1 == 0.1);

    let Pair(integer, decimal) = pair;

    assert!(integer == 1);
    assert!(decimal == 0.1);
}