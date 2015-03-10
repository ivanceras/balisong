//ray.rs

use vector::Vector;
use point::Point;
use std::fmt;

pub struct Ray{
	origin:Point,
	unit_dir: Vector, //unit vector of direction
}

impl Ray{

	pub fn new(origin:&Point, direction:Vector)->Ray{
		let unit_dir = direction.unit_vector();
		Ray{origin:origin.clone(), unit_dir: unit_dir}
		
	}
	
	pub fn at_length(&self, length:u64)->Vector{
		let xlen = self.origin.x as f64 + self.unit_dir.x * length as f64; 
		let ylen = self.origin.y as f64 + self.unit_dir.y * length as f64;
		let zlen = self.origin.z as f64 + self.unit_dir.z * length as f64;
		Vector::new(xlen, ylen, zlen)

	}
	
}


impl fmt::Display for Ray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "origin: {}, direction:{})", self.origin, self.unit_dir)
    }
}

