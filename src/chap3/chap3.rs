// A struct with two fields
#[derive(PartialEq, Debug)] // Allows struct comparisons in tests
pub struct Point {
    pub x: f32,
    pub y: f32,
}


// Structs can be reused as fields of another struct
#[derive(PartialEq, Debug)]
pub struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    pub top_left: Point,
    pub bottom_right: Point,
}

// Add a function rect_area which calculates the area of a Rectangle (try using nested destructuring).
// Add a function square which takes a Point and a f32 as arguments, and returns a Rectangle with its top left corner on the point, and a width and height corresponding to the f32.
pub fn rect_area(input_rectangle: Rectangle) -> f32 {
    let Rectangle { top_left: Point {x: top_x, y: top_y}, bottom_right: Point { x: bottom_x, y: bottom_y}} = input_rectangle;
    (bottom_x - top_x) * (top_y - bottom_y)
}

pub fn square(top_left: Point, edge: f32) -> Rectangle {
    let Point { x: top_x, y: top_y } = top_left;
    Rectangle {
        top_left,
        bottom_right: Point {
            x: top_x + edge,
            y: top_y - edge,
        }
    }
}