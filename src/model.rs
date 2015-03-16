use point::Point;
use octree::Octree;
use std::fmt;

//an octree when put in a scene
pub struct Model{
	pub location:Point,
	pub octree:Octree<bool>,
	pub scale:f64,
}


impl Model{
	
	pub fn new(location:Point, octree:Octree<bool>, scale:f64)->Model{
		Model{location:location, octree:octree, scale:scale}
	}
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "location: {}, scale:{}, octree: {}", self.location, self.scale, self.octree)
    }
}