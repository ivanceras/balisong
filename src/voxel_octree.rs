use voxel::Voxel;
use std::option::Option;
use std::fmt;
use location;
use color::Color;

//TODO: 
//memory optimization: check to see if all 8 children in a voxel has same materials, then simplify 
//by removing all children and store the information in the parent
//
pub struct Octree{
	value:u8,
	voxel:Option<Voxel>,//optional content
	children:Vec<Octree>//leaf only when there is no children content, children.len() == 0,
}


impl Octree{
	
	pub fn new()->Octree{
		Octree{value:0, voxel:None, children:Vec::new()}
	}
	
	pub fn put_tree(&mut self, location:Vec<u8>, voxel:Voxel){
		let mut m_location = location.clone();
		self.put_tree_internal(&mut m_location, &voxel)
	}
	
	//recursive call without recursive cloning the location
	fn put_tree_internal(&mut self, location:&mut Vec<u8>, voxel:&Voxel){
		let root_loc = location[0];
		if self.is_empty(root_loc){
			self.put(root_loc, None);
		}
		let node = self.get_as_mut(root_loc);//here is the node
		if location.len() == 1 {//this is the last
			let last = location.len() - 1;
			node.put(location[last], Some(voxel.clone()));
		}
		location.remove(0);
		if location.len() > 0 {
			node.put_tree_internal(location, &voxel);
		}
	}
	
	pub fn get_voxel(&self, location:Vec<u8>)->Voxel{
		let octree = self.get_tree(location);
		if octree.voxel.is_some(){
			return octree.voxel.clone().unwrap();
		}
		else{
			println!("octree.voxel is None");
		}
		panic!("No voxel content!");
	}
	
	fn get_tree(&self, location:Vec<u8>)->&Octree{
		let mut m_location = location.clone();
		self.get_tree_internal(&mut m_location)
	}
	
	//recursive call without recursive cloning the location
	fn get_tree_internal(&self, location:&mut Vec<u8>)->&Octree{
		let root_loc = location[0];
		let node = self.get(root_loc);
		if location.len() == 1 {
			return node;
		}
		location.remove(0);
		let child_node = node;
		if location.len() > 0 {
			let child_node = child_node.get_tree_internal(location);
			return child_node;
		}
		panic!("Shouldn't reach here!");
	}
	
	
	pub fn put(&mut self, location:u8, voxel:Option<Voxel>){
		//println!("putting voxel..at {}, location: {:8b}", self, location);
		let index = self.index_of(location);
		if self.is_empty(location){
			self.value = self.value | location;
			self.children.push(Octree::new());
			self.voxel = voxel;
			//println!("\tafter put: {}", self);
		}
		else{
			println!("Replacing {}", index);
		}
	}
	
	
	//return the octree at this location
	fn get(&self, location:u8)->&Octree{
		//println!("LEAF: {}",self);
		if self.is_occupied(location){
			let index = self.index_of(location);
			return &self.children[index];
		}
		else{
			panic!("No octree at location: {:8b}",location);
		}
	}
	//get the octree as mutable
	fn get_as_mut(&mut self, location:u8)->&mut Octree{
		if self.is_occupied(location){
			let index = self.index_of(location);
			return &mut self.children[index];
		}
		else{
			panic!("No octree at location: {:8b}",location);
		}
	}
	
	fn is_occupied(&self, location:u8)->bool{
		self.value & location == location
	}
	
	fn is_empty(&self, location:u8)->bool{
		!self.is_occupied(location)
	}
	
	//give the previous value, determine where in the children to insert
	fn index_of(&self, location:u8)->usize{
		let mut index = 0;
		for i in 0..8{
			let byte = 1 << i;
			if byte == location {
				return index;
			}
			if self.value & byte == byte{
				index += 1;
			}
		}
		return index;
	}
	//checks if path of the octree exist or not
	pub fn is_point_occupied(&self, lod:u8, x:i64, y:i64 ,z:i64)->bool{
		if location::is_bounded(lod, x, y, z){
			let loc = location::from_xyz(lod, x as u64, y as u64, z as u64);
			return self.is_location_occupied(&loc);
		}
		false
	}		

	//checks if path of the octree exist or not
	pub fn is_location_occupied(&self, location:&Vec<u8>)->bool{
		let mut m_location = location.clone();
		self.is_location_occupied_internal(&mut m_location)
	}
	
	fn is_location_occupied_internal(&self, location:&mut Vec<u8>)->bool{
		let root_loc = location[0];
		if self.is_empty(root_loc){
			return false;
		}
		else{
			let node = self.get(root_loc);
			if location.len() == 1 {
				let last = location.len() - 1;//actually location[last] = location[0] = root_loc
				return node.is_occupied(location[last]);
			}
			location.remove(0);
			let child_node = node;
			if location.len() > 0 {
				return child_node.is_location_occupied_internal(location);
			}
			panic!("Shouldn't reach here!");
		}
	}
	
	pub fn get_color(&self, lod:u8, x:i64, y:i64 ,z:i64)->Color{
		let loc = location::from_xyz(lod, x as u64, y as u64, z as u64);
		let voxel = self.get_voxel(loc);
		voxel.color
	}
	
}


impl fmt::Display for Octree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(value: {:8b}), children: {}",self.value, self.children.len())
    }
}