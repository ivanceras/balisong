//point.rs

use std::fmt;

pub struct Point{
	pub x:i64,
	pub y:i64,
	pub z:i64
}

impl Point{
    pub fn new(x:i64, y:i64, z:i64)->Point{
        Point{x:x, y:y, z:z}
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Clone for Point {
    fn clone(&self) -> Point { Point{x:self.x, y:self.y, z:self.z} }
}
