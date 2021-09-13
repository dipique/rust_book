use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32, y: i32
}

// Add has a generic default type
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Here's an implementation that doesn't use the default
#[derive(Debug, Copy, Clone, PartialEq)]
struct Millimeters(u32);
#[derive(Debug, Copy, Clone, PartialEq)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

pub fn run() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    assert_eq!(
        Millimeters(23) + Meters(1),
        Millimeters(1023)
    );
}