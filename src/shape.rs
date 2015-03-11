//shape.rs
use point::Point;
use std::num::Float;
use vector::Vector;
use normal::Normal;


pub trait Shape{
    fn is_inside(&self, x:i64, y:i64, z:i64)->bool;
    fn normal(&self, x:i64, y:i64, z:i64)->Normal;
    fn name(&self)->String;
}


pub struct Sphere{
    radius:u64,
    center:Point
}

pub struct Cube{
    lower:Point,
    upper:Point
}

struct Prism{
    bound1:Point,//is inside when x >= bound1.x && x <= bound2.x
    bound2:Point,//               y >= bound1.y && y <= bound2.y
                 //               z >= bound1.z && z <= bound2.z     
}

struct Cylinder{
    radius:u64,
    height:u64,
}



impl Sphere{
    pub fn new(radius:u64, center:&Point)->Sphere{
        Sphere{radius:radius, center:center.clone()}
    }
    
    
}

impl Shape for Sphere{

     fn is_inside(&self, x:i64, y:i64, z:i64)->bool{
        let xf = (x - self.center.x) as f64;
		let yf = (y - self.center.y) as f64;
		let zf = (z - self.center.z) as f64;
		let rad = (xf*xf + yf*yf + zf*zf).sqrt().round() as u64;
		
		if rad <= self.radius {//solid
		//if rad == self.radius {//thin carved
		//if rad <= self.radius && rad >= (self.radius - 1)  {//carved out, 2 inner walls down
			return true;
		}
		false
    }
    
    fn normal(&self, x:i64, y:i64, z:i64)->Normal{
        let p = Vector::new(x as f64, y as f64, z as f64);
        let cp = p.subtract_point(&self.center);
        let normal = Normal::from_vector(&cp);
        normal
    }
    
    fn name(&self)->String{
        "sphere".to_string()
    }
}

impl Cube{
    pub fn new(radius:u64, center:&Point)->Cube{
        
        let xlower = center.x - radius as i64;
        let ylower = center.y - radius as i64;
        let zlower = center.z - radius as i64;
        
        let xupper = center.x + radius as i64;
        let yupper = center.y + radius as i64;
        let zupper = center.z + radius as i64;
        
        let lower = Point::new(xlower, ylower, zlower);
        let upper = Point::new(xupper, yupper, zupper);
        
        Cube{lower:lower, upper:upper}
    }
    
    fn is_inside_priv(&self, x:i64, y:i64, z:i64)->bool{
		if x >= self.lower.x && x <= self.upper.x &&
           y >= self.lower.y && y <= self.upper.y &&
		   z >= self.lower.z && z <= self.upper.z {//solid
			return true;
		}
	   false
	}
    
    fn is_outside_priv(&self, x:i64, y:i64, z:i64, inset:i64)->bool{
		if x < (self.lower.x + inset) || x > (self.upper.x - inset) ||
           y < (self.lower.y + inset) || y > (self.upper.y - inset) ||
		   z < (self.lower.z + inset) || z > (self.upper.z - inset) {//solid
			return true;
		}
	   false
	}
}



impl Shape for Cube{

    fn is_inside(&self, x:i64, y:i64, z:i64)->bool{
		self.is_inside_priv(x,y,z) && self.is_outside_priv(x,y,z,1)
    }
    
    
    fn normal(&self, x:i64, y:i64, z:i64)->Normal{
        let p = Vector::new(0.0, 0.0, 0.0);
        let normal = Normal::from_vector(&p);
        normal
    }
    fn name(&self)->String{
        "cube".to_string()
    }
}
