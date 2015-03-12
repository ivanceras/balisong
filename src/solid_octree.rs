use std::fmt;
use location;
use color::Color;

//TODO: 
//memory optimization: check to see if all 8 children in a voxel has same materials, then simplify 
//by removing all children and store the information in the parent
//
pub struct Octree{
	pub value:u8,
	pub solid:bool, //determines if the whole octree is solid or not
	pub children:Vec<Octree>//leaf only when there is no children content, children.len() == 0,
}


impl Octree{
	
	pub fn new()->Octree{
		Octree{value:0, children:Vec::new(), solid:false}
	}
	
	pub fn set_tree(&mut self, location:Vec<u8>){
		let mut m_location = location.clone();
		self.set_tree_internal(&mut m_location)
	}
	
	//recursive call without recursive cloning the location
	fn set_tree_internal(&mut self, location:&mut Vec<u8>){
		let root_loc = location[0];
		if self.is_empty(root_loc){
			self.set(root_loc);
		}
		let node = self.get_as_mut(root_loc);//here is the node
		if location.len() == 1 {//this is the last
			let last = location.len() - 1;
			assert!(last == 0);
			node.set(location[last]);
			//println!("last is called: {:8b}, children: {}",node.value, node.children.len());
		}
		
		location.remove(0);
		if location.len() > 0 {
			node.set_tree_internal(location);
		}
	}
	
	/* TODO: this must be implemented somewhere
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
	*/
	
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
	
	
	//TODO: if 8 children at this location is set with same material, unset this location
	// then traverse to the parent location via `location.remove(last)`, then set the parent material there
	//other optimization: Whatever has the most material count in a voxel wins, then gets to set the parent
	//other option: Blending of materials, blend the material properties which makes it homogeneous material.
	//use only blending when all of the other material is different from each other.
	//50%water + 50%dirt = mud
	//90%water + 10%dirt = murky water
	//air + dirt = dust
	//fire + dirt = lava
	//cement + water = concrete
	//titanium + aluminum = alloy, hardness of material is recalculated
	//iron + oxygen = rust
	
	pub fn set(&mut self, location:u8){
		let index = self.index_of(location);
		if self.is_empty(location){
			self.value = self.value | location;
			self.children.push(Octree::new());
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
	
	pub fn is_all_children_solid(&self)->bool{
		let mut cnt = 0;
		for i in 0..self.children.len(){
			if self.children[i].solid {
				cnt += 1;
			}else{
				return false;
			} 
		}
		if cnt == 8 {
			return true;
		}
		false
	}
	pub fn is_solid(&self)->bool{
		self.solid
	}
	
	//determine whether this node is already a leaf
	fn is_leaf(&self)->bool{
		self.children.len() == 0
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
			if node.is_solid(){
				return true;
			}
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

}


impl fmt::Display for Octree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(value: {:8b}), children: {}",self.value, self.children.len())
    }
}