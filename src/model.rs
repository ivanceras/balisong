use point::Point;
use octree::Octree;
use std::fmt;

use normal::Normal;

//an octree when put in a scene
pub struct Model{
	pub location:Point,
	pub octree:Octree<bool>,
	pub normal:Octree<Normal>,
	pub scale:f64,
}


impl Model{
	
	pub fn new(location:Point, octree:Octree<bool>, normal:Octree<Normal>, scale:f64)->Model{
		Model{location:location, octree:octree, normal:normal, scale:scale}
	}
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "location: {}, scale:{}, octree: {}", self.location, self.scale, self.octree)
    }
}