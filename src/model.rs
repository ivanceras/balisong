use point::Point;
use voxtree::Voxtree;
use std::fmt;

use normal::Normal;

//an Voxtree when put in a scene
pub struct Model{
	pub location:Point,
	pub voxtree:Voxtree<bool>,
	pub normal:Voxtree<Normal>,
	pub scale:f64,
}


impl Model{
	
	pub fn new(location:Point, voxtree:Voxtree<bool>, normal:Voxtree<Normal>, scale:f64)->Model{
		Model{location:location, voxtree:voxtree, normal:normal, scale:scale}
	}
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "location: {}, scale:{}, Voxtree: {}", self.location, self.scale, self.voxtree)
    }
}