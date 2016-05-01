use std::fmt;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64
}

impl Point {
    fn midpoint(&self, other: &Point) -> Point {
        Point {
            x: self.x + (other.x - self.x) / 2f64,
            y: self.y + (other.y - self.y) / 2f64
        }
    }

    fn dist2 (&self, other: &Point) -> f64 {
        (other.x - self.x).powi(2) + (other.y - self.y).powi(2)
    }

    fn dist(&self, other: &Point) -> f64 {
        Point::dist2(&self, &other).sqrt()
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.p1, self.p2)
    }
}

impl Rectangle {
    fn new(x1: f64, y1: f64, x2: f64, y2: f64) -> Rectangle {
        Rectangle {
            p1: Point {x: x1, y: y1},
            p2: Point {x: x2, y: y2}
        }
    }

    fn center(&self) -> Point {
        self.p1.midpoint(&self.p2)
    }

    fn distance(&self, other: &Rectangle) -> f64 {
        Point::dist(&self.center(), &other.center())
    }
}

fn rect() {
    let r1 = Rectangle::new(0f64, 0f64, 1f64, 1f64);
    let r2 = Rectangle::new(1f64, 0f64, 2f64, 1f64);
    println!("R1: {}", r1);
    println!("R2: {}", r2);
    println!("Distance: {}", r1.distance(&r2));
}

fn main() {
    rect();
}
