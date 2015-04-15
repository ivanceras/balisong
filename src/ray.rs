//ray.rs

use vector::Vector;
use point::Point;
use std::fmt;
use lod::LOD;

pub struct Ray{
	origin:Point,
	unit_dir: Vector, //unit vector of direction
}

impl Ray{

	pub fn new(origin:&Point, direction:&Vector)->Ray{
		let unit_dir = direction.unit_vector();
		Ray{origin:origin.clone(), unit_dir: unit_dir}
		
	}
	
	pub fn at_length(&self, length:f64)->Vector{
		let xlen = self.origin.x as f64 + self.unit_dir.x * length; 
		let ylen = self.origin.y as f64 + self.unit_dir.y * length;
		let zlen = self.origin.z as f64 + self.unit_dir.z * length;
		Vector::new(xlen, ylen, zlen)

	}
	
	pub fn at_lod_length(&self, lod:&LOD, view_lod:&LOD, length:f64)->Vector{
		let limit = lod.limit as f64;
		let view_limit = view_lod.limit as f64;
		let view_scale = view_limit / limit; 
		let xlen = (self.origin.x as f64 + self.unit_dir.x * length) * view_scale; 
		let ylen = (self.origin.y as f64 + self.unit_dir.y * length) * view_scale;
		let zlen = (self.origin.z as f64 + self.unit_dir.z * length) * view_scale;
		Vector::new(xlen, ylen, zlen)

	}
	
}


impl fmt::Display for Ray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "origin: {}, direction:{})", self.origin, self.unit_dir)
    }
}

