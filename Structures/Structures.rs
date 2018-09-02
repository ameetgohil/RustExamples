use std::fmt;

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

// A unit struct
struct Nil;

// A Tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug, Copy)]
struct Point {
    x: f32,
    y: f32,
}


impl Clone for Point {
    fn clone(&self) -> Point {
        *self
    }
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "p1: {} {} p2:{} {}", self.p1.x, self.p1.y, self.p2.x, self.p2.y)
    }
}

fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle { p1: Point {x: x1, y: y1}, p2: Point {x: x2, y: y2} } = rect;
    (x1-x2)*(y1-y2)
}

fn square(origin: Point, length: f32) -> Rectangle {
    
    Rectangle {
        p1: origin,
        p2: Point { x: origin.x + length, y: origin.y + length }
    }
}

fn main() {
    // Create struct with fields init shorthand
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    //Instantiate a `Point`
    let point: Point = Point { x: 0.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our other one
    let new_point = Point {x: 0.1, ..point };
    // `new_point.y` will be the same as `point.y` because we used that fields from `point`
    println!("second point: ({}, {})", new_point.x, new_point.y);

    // Destructure the point sing a `let`binding
    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        //struct instantiation is an expression too
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };

    // Instantiate a unit struct
    let _nil = Nil;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    //Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    //Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("Area of rectangle is {}" , rect_area(_rectangle));

    let sq_origin = Point { x: 1.0, y: 2.0 };
    let sq_len = 32.5;
    println!("Point {:?} with length: {}, Square: {}",sq_origin, sq_len, square(sq_origin, sq_len));
             
}
    
