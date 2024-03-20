use crate::*;
use chap3::chap3::{rect_area, square, Rectangle, Point};


#[test]
fn test_rectangle() {
    assert_eq!(rect_area(Rectangle{ top_left: Point{x: 5.0, y: 15.0}, bottom_right: Point{x: 10.0, y:0.0}}), 5.0*15.0);

    assert_eq!(rect_area(Rectangle{ top_left: Point{x: 1.0, y: 1.0}, bottom_right: Point{x: 1.0, y:1.0}}), 0.0);

    assert_eq!(rect_area(Rectangle{ top_left: Point{x: -5.0, y: 15.0}, bottom_right: Point{x: 10.0, y:-5.0}}), 15.0*20.0);
}

#[test]
fn test_square() {
    assert_eq!(square(Point{x: 5.0, y: 15.0}, 5.0), Rectangle{ top_left: Point{x: 5.0, y: 15.0}, bottom_right: Point{x: 10.0, y:10.0}});

    assert_eq!(square(Point{x: 1.0, y: 1.0}, 0.0), Rectangle{ top_left: Point{x: 1.0, y: 1.0}, bottom_right: Point{x: 1.0, y:1.0}});

    assert_eq!(square(Point{x: -5.0, y: 15.0}, 10.0), Rectangle{ top_left: Point{x: -5.0, y: 15.0}, bottom_right: Point{x: 5.0, y:5.0}});
}