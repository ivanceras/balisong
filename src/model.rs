use point::Point;
use voxtree::Voxtree;
use std::fmt;

use normal::Normal;

//an Voxtree when put in a scene
pub struct Model{
	pub location:Point,
	pub normal:Voxtree<Normal>,
	pub scale:f64,
}


impl Model{
	
	pub fn new(location:Point, normal:Voxtree<Normal>, scale:f64)->Model{
		Model{location:location, normal:normal, scale:scale}
	}
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "location: {}, scale:{}, normals: {}", self.location, self.scale, self.normal)
    }
}