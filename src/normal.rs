//normal.rs

use std::fmt;
use vector::Vector;

///
/// This is a vector expressed in 3 i8's instead of 3 f64's
///
pub struct Normal{
    pub x:i8,
    pub y:i8,
    pub z:i8
}

impl Normal{

    
    pub fn new(x:i8, y:i8, z:i8)->Normal{
        Normal{x:x, y:y, z:z}
    }
    
    pub fn from_vector(v:&Vector)->Normal{
        let uv = v.unit_vector();
        let i8x = (uv.x * 127.0).round() as i8;
        let i8y = (uv.y * 127.0).round() as i8;
        let i8z = (uv.z * 127.0).round() as i8;
        Normal{x:i8x, y:i8y, z:i8z}
    }
    
    pub fn from_f64(x:f64, y:f64, z:f64)->Normal{
        Normal::from_vector(&Vector::new(x,y,z))
    }
    
    pub fn unit_vector(&self)->Vector{
        let x = self.x as f64;
        let y = self.y as f64;
        let z = self.z as f64;
        let v = Vector::new(x, y, z);
        v.unit_vector()
    }
}

impl fmt::Display for Normal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Clone for Normal {
    fn clone(&self) -> Normal { Normal{x:self.x, y:self.y, z:self.z} }
}
