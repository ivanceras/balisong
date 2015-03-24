//vector.rs
use std::num::Float;
use std::fmt;
use point::Point;


///
/// There are a multitude of vector algebra implementations out there
/// but it is a good thing to learn in action
///
///
pub struct Vector{
	pub x:f64,
	pub y:f64,
	pub z:f64,
}

impl Vector{

    pub fn new(x:f64, y:f64, z:f64)->Vector{
        Vector{x:x, y:y, z:z}
    }
    pub fn from_point(point:&Point)->Vector{
        Vector{x:point.x as f64, y:point.y as f64, z:point.z as f64}
    }
	pub fn distance(&self)->f64{
		(self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
	}
	
	pub fn unit_vector(&self)->Vector{
		let d = self.distance();
		let xnew = self.x / d;
		let ynew = self.y / d;
		let znew = self.z / d;
		Vector{x:xnew, y:ynew, z:znew}
	}
	
	pub fn scale(&self, scale:f64)->Vector{
		Vector{x:self.x * scale, y:self.y * scale, z:self.z * scale}
	}
	
	pub fn negate(&self)->Vector{
		Vector{x:self.x * -1.0, y:self.y * -1.0, z:self.z * -1.0}
	}
	
	pub fn subtract_point(&self, point:&Point)->Vector{
		let x = self.x - point.x as f64;
		let y = self.y - point.y as f64;
		let z = self.z - point.z as f64;
		Vector::new(x,y,z)
	}
	pub fn add_point(&self, point:&Point)->Vector{
		let x = self.x + point.x as f64;
		let y = self.y + point.y as f64;
		let z = self.z + point.z as f64;
		Vector::new(x,y,z)
	}
	
	pub fn add(&self, vector:&Vector)->Vector{
		let x = self.x + vector.x as f64;
		let y = self.y + vector.y as f64;
		let z = self.z + vector.z as f64;
		Vector::new(x,y,z)
	}
	
	pub fn subtract(&self, vector:&Vector)->Vector{
		let x = self.x - vector.x as f64;
		let y = self.y - vector.y as f64;
		let z = self.z - vector.z as f64;
		Vector::new(x,y,z)
	}
	
	// http://freespace.virgin.net/hugo.elias/routines/r_dot.htm
	//  DotProduct = (x1*x2 + y1*y2 + z1*z2)
	pub fn dot(&self, vec:&Vector)->f64{
		self.x * vec.x + self.y * vec.y + self.z * vec.z
	}
	
	//http://freespace.virgin.net/hugo.elias/routines/r_cross.htm
	//ox = (y1 * z2) - (y2 * z1)
	//oy = (z1 * x2) - (z2 * x1)
	//oz = (x1 * y2) - (x2 * y1)
	pub fn cross(&self, vec:&Vector)->Vector{
		let x = self.y * vec.z - vec.y * self.z;
		let y = self.z * vec.x - vec.z * self.x;
		let z = self.x * vec.y - vec.x * self.y;
		Vector{x:x, y:y, z:z}
	}
	
    //http://stackoverflow.com/questions/13275719/rotate-a-3d-point-around-another-one
    //http://stackoverflow.com/questions/16380147/how-to-rotate-an-object-defined-by-x-y-z-points-around-the-x-y-or-z-axis
    //http://stackoverflow.com/questions/14607640/rotating-a-vector-in-3d-space
    pub fn rotate_at_z(&self, a:f64)->Vector{
    	let x = self.x * a.cos() - self.y * a.sin();
		let y = self.x * a.sin() + self.y * a.cos();
		let z = self.z;  
		Vector::new(x,y,z)
    }
     pub fn rotate_at_y(&self, b:f64)->Vector{
		let x = self.z * b.sin() + self.x * b.cos();
		let y = self.y;
		let z = self.z * b.cos() - self.x * b.sin();  
		Vector::new(x,y,z)
    }
    pub fn rotate_at_x(&self, c:f64)->Vector{
		let x = self.x;
    	let y = self.y * c.cos() - self.z * c.sin();
		let z = self.y * c.sin() + self.z * c.cos();
		Vector::new(x,y,z)
    }
    
    pub fn as_point(&self)->Point{
		Point::new(self.x.round() as i64, self.y.round() as i64, self.z.round() as i64)
	}
    
 
	
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Clone for Vector {
    fn clone(&self) -> Vector { Vector{x:self.x, y:self.y, z:self.z} }
}
