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
    upper:Point,
    center:Point,
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
		
		//if rad <= self.radius {//solid, caution: uses a lot of memory
		//if rad == self.radius {//very thin carved
		if rad <= self.radius && rad >= (self.radius - 1)  {//carved out, 2 inner walls down
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
        
        Cube{lower:lower, upper:upper, center:center.clone()}
    }
    
    fn is_inside_private(&self, x:i64, y:i64, z:i64)->bool{
		if x >= self.lower.x && x <= self.upper.x &&
           y >= self.lower.y && y <= self.upper.y &&
		   z >= self.lower.z && z <= self.upper.z {//solid
			return true;
		}
	   false
	}
    
    fn is_outside_private(&self, x:i64, y:i64, z:i64, inset:i64)->bool{
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
		self.is_inside_private(x,y,z) && self.is_outside_private(x,y,z,1)//carved
		//self.is_inside_private(x,y,z)//solid
    }
    
    //TODO:
    // There are 6 different scenrio
    //
    // right  1,  0,  0  x <= self.upper.x && x > self.upper.x - inset
    // left  -1,  0,  0  x >= self.lower.x && x < self.lower.x - inset
    // up     0,  1,  0  y <= self.upper.y && y > self.upper.y - inset
    // down   0, -1,  0  y >= self.lower.y && y < self.lower.y - inset
    // front  0,  0,  1  z <= self.upper.y && z > self.upper.z - inset
    // back   0,  0, -1  z >= self.lower.z && z < self.lower.z - inset

    fn normal(&self, x:i64, y:i64, z:i64)->Normal{
    	let inset = 1;
        if x <= self.upper.x && x > self.upper.x - inset{//right
			let vec = Vector::new(1.0, 0.0, 0.0);
			return Normal::from_vector(&vec);
        }
		else if x >= self.lower.x && x < self.lower.x - inset{//left
			let vec = Vector::new(-1.0, 0.0, 0.0);
			return Normal::from_vector(&vec);
		}
		else if y <= self.upper.y && y > self.upper.y - inset{//up
			let vec = Vector::new(0.0, 1.0, 0.0);
			return Normal::from_vector(&vec);
		}
		else if y >= self.lower.y && y < self.lower.y - inset{//down
			let vec = Vector::new(0.0, -1.0, 0.0);
			return Normal::from_vector(&vec);
		}
		else if z <= self.upper.y && z > self.upper.z - inset{//front
			let vec = Vector::new(0.0, 0.0, 1.0);
			return Normal::from_vector(&vec);
		}
		else if z >= self.lower.z && z < self.lower.z - inset{//back
			let vec = Vector::new(0.0, 0.0, -1.0);
			return Normal::from_vector(&vec);
		}
		let vec = Vector::new(-1.0, 0.0, 0.0);
		return Normal::from_vector(&vec);
    }

/*
    fn normal(&self, x:i64, y:i64, z:i64)->Normal{
       let p = Vector::new(x as f64, y as f64, z as f64);
        let cp = p.subtract_point(&self.center);
        let normal = Normal::from_vector(&cp);
        normal
    }
*/
    fn name(&self)->String{
        "cube".to_string()
    }
}
