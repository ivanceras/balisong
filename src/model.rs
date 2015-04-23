use point::Point;
use voxel::voxtree::Voxtree;
use std::fmt;

use normal::Normal;

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

