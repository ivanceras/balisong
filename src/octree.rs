use std::fmt;
use location;

//TODO: 
//memory optimization: check to see if all 8 children in a voxel has same materials, then simplify 
//by removing all children and store the information in the parent
//
pub struct Octree<T>{
	pub bitset:u8,//contains the information of which of octants belongs to 
	pub content:Option<T>,
	pub children:Vec<Octree<T>>//leaf only when there is no children content, children.len() == 0,
}


impl <T> Octree<T>{
	
	pub fn new()->Octree<T>{
		Octree{bitset:0, content: None, children:Vec::new()}
	}

	/// 
	/// Sets the content of the octree at this location
	/// 
	pub fn set_tree(&mut self, location:Vec<u8>, content:Option<T>){
		let mut m_location = location.clone();
		self.set_tree_internal(&mut m_location, content)
	}
	
	
	///
	/// Internal implementation for setting the octree
	///
	
	fn set_tree_internal(&mut self, location:&mut Vec<u8>, content:Option<T>){
		if self.is_empty(location[0]){//if empty, create a node with None content
			self.set(location[0], None);
		}
		let node = self.get_as_mut(location[0]);//here is the node
		if location.len() > 1 {//this is the last
			location.remove(0);
			if location.len() > 0 {
				node.set_tree_internal(location, content);
			}
		}
		else{//location.len() == 1
			assert!(location.len() == 1, "This should be the last location");
			node.set(location[0], content);
		}
	}

	
	///
	///Traverse the tree and get the node at this location
	///
	///
	pub fn get_tree(&self, location:Vec<u8>)->&Octree<T>{
		let mut m_location = location.clone();
		self.get_tree_internal(&mut m_location)
	}
	
	///
	///internal implementation of getting the octree
	///
	fn get_tree_internal(&self, location:&mut Vec<u8>)->&Octree<T>{
		let node = self.get_node(location[0]);
		if location.len() > 1 {
			location.remove(0);
			return node.get_tree_internal(location);
		}
		else{
			assert!(location.len() == 1, "This should be the last location");
			return node;
		}
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
	
	///
	/// Set the content of this node at bit location
	/// TODO; if there is already a content, overwrite it
	///
	pub fn set(&mut self, location:u8, content:Option<T>){
		self.children.push(Octree::new());
		self.bitset = self.bitset | location;
		self.content = content;
	}

	///
	/// get the child node located  
	///
	fn get_node(&self, location:u8)->&Octree<T>{
		if self.is_occupied(location){
			let index = self.index_of(location);
			return &self.children[index];
		}
		else{
			panic!("No octree at location: {:8b}",location);
		}
	}
	
	///
	/// Get the node as mutable at this location
	///
	///
	
	fn get_as_mut(&mut self, location:u8)->&mut Octree<T>{
		if self.is_occupied(location){
			let index = self.index_of(location);
			return &mut self.children[index];
		}
		else{
			panic!("No octree at location: {:8b}",location);
		}
	}
	
	///
	/// checks whether this node is set or not
	///
	fn is_occupied(&self, location:u8)->bool{
		self.bitset & location == location
	}
	
	///
	/// checks whether the node has value or not
	///
	fn is_empty(&self, location:u8)->bool{
		!self.is_occupied(location)// or self.bitset == 0
	}
	
	///
	///it is a leaf when there is no children
	///
	///
	
	fn is_leaf(&self)->bool{
		self.children.len() == 0
	}
	
	///
	/// get the actual index of the child from the vector base on bitset value, 
	/// by which the actual vector array is sparsed
	///
	fn index_of(&self, location:u8)->usize{
		let mut index = 0;
		for i in 0..8{
			let byte = 1 << i;
			if byte == location {
				return index;
			}
			if self.bitset & byte == byte{
				index += 1;
			}
		}
		return index;
	}

	///
	/// check whether the certain point in 3D space is occupied or not
	/// This uses the conversion of the implementation of location::from_xyz
	/// to do the calculation
	/// TODO: create a more simple/optimum algorithm for this
	///
	pub fn is_point_occupied(&self, lod:u8, x:i64, y:i64 ,z:i64)->bool{
		if location::is_bounded(lod, x, y, z){ //no more bounds check if the camera is located inside the one-world octree
			let loc = location::from_xyz(lod, x as u64, y as u64, z as u64);
			return self.is_location_occupied(&loc);
		}
		false
	}		

	///
	/// check whether the a certain location is occupied or not, expressed in 
	/// location notation which is just an arrray of 8bit values which describes the location of the 
	/// voxel at each LOD (level of detail )
	///
	pub fn is_location_occupied(&self, location:&Vec<u8>)->bool{
		let mut m_location = location.clone();
		self.is_location_occupied_internal(&mut m_location)
	}
	
	///
	/// private implementation, since location is mutated at each recursive pass, check whether the a certain location is occupied or not, expressed in 
	/// location notation which is just an arrray of 8bit values which describes the location of the 
	/// voxel at each LOD (level of detail )
	///
	fn is_location_occupied_internal(&self, location:&mut Vec<u8>)->bool{
		if self.is_empty(location[0]){
			return false;
		}
		else{
			let node = self.get_node(location[0]);
			if location.len() > 1 {
				location.remove(0);
				return node.is_location_occupied_internal(location);
			}
			else{// location.len() == 1
				assert!(location.len() == 1, "This should be the last location array");
				return node.is_occupied(location[0]);
			}
			panic!("Shouldn't reach here!");
		}
	}
	
	///
	/// return the total number of leaf nodes in the octree
	//
	pub fn count_leaf(&self)->u64{
		self.count_leaf_internal(self)
	}
	
	/// 
	/// internal implementation of counting the leaf nodes in the octree
	/// 
	fn count_leaf_internal(&self, node:&Octree<T>)->u64{
		let mut count = 0;
		if node.is_leaf(){
			count += 1;
		}
		for i in 0..node.children.len(){
			count += self.count_leaf_internal(&node.children[i]);
		}
		count
	}
	
	///
	/// Count the total number of nodes
	///
	///
	pub fn count_nodes(&self)->u64{
		self.count_nodes_internal(self)
	}
	
	///
	/// internal implementation for counting the total number of nodes
	///
	fn count_nodes_internal(&self, node:&Octree<T>)->u64{
		//let mut count = location::count_bits(node.bitset) as u64;
		let mut count = node.children.len() as u64;
		for i in 0..node.children.len(){
			count += self.count_nodes_internal(&node.children[i]);
		}
		count
	}
	
	///
	/// checks if all children of this node are all leaf nodes
	///
	pub fn is_all_children_leaf(&self)->bool{
		let mut count = 0;
		for i in 0..self.children.len(){
			if self.children[i].is_leaf(){
				count += 1;
			}
			else{
				return false;
			}
		}
		count == 8
	}
	
	///
	/// checks if all children of this octree is fully occupied
	///
	pub fn is_solid(&self)->bool{
		self.is_solid_internal(self)
	}
	
	/// traverse to the children then check until it hit the leaf
	/// if this fully filed and all the chilren are fully filed until it hits the leaf, then it is sold
	fn is_solid_internal(&self, node:&Octree<T>)->bool{
		let partially_solid = node.is_partial_solid();
		println!("\t partially_solid: {}",partially_solid);
		let mut count = 0;
		if node.is_all_children_leaf(){
			println!("\t leaf!");
			return partially_solid;
		}
		for i in 0..node.children.len(){
			if self.is_solid_internal(&node.children[i]){
				count += 1;
			}
			else{
				println!("\t not solid!");
				return false;
			}
		}
		println!("\t count : {}",count);
		partially_solid && count == 8
	}
	
	pub fn is_partial_solid(&self)->bool{
		self.bitset == 255
	}
	
	pub fn count_solids(&self)->u64{
		self.count_solids_internal(self)
	}
	
	fn count_solids_internal(&self, node:&Octree<T>)->u64{
		let mut count = 0;
		if node.is_solid(){
			count += 1;
		}
		for i in 0..node.children.len(){
			if node.children[i].is_solid(){
				count += 1;
			}
		}
		count
	}

}


impl <T: fmt::Display> fmt::Display for Octree<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(bitset: {:8b}), children: {}",self.bitset, self.children.len())
    }
}