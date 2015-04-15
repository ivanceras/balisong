use std::fmt;

use constants;


pub struct Voxtree<T>{
	pub bitset:u64,//contains the information of which of sector belongs to 
	pub content:Option<T>,
	pub children:Vec<Voxtree<T>>,//leaf only when there is no children content, children.len() == 0,
}


impl <T> Voxtree<T>{
	
	pub fn new()->Voxtree<T>{
		Voxtree{bitset:0, content: None, children:Vec::new()}
	}

	/// 
	/// Sets the content of the Voxtree at this location
	/// 
	pub fn set_tree(&mut self, location:&Vec<u64>, content:&mut Option<T>){
		let mut m_location = location.clone();
		self.set_tree_recursive(&mut m_location, content)
	}
	
	
	///
	/// Internal implementation for setting the Voxtree
	///
	
	fn set_tree_recursive(&mut self, location:&mut Vec<u64>, content:&mut Option<T>){
		self.set_path(location[0]);
		let node = self.get_as_mut(location[0]);//here is the node
		if location.len() > 1 {//this is the last
			location.remove(0);
			if location.len() > 0 {
				node.set_tree_recursive(location, content);
			}
		}
		else{//location.len() == 1
			assert!(location.len() == 1, "This should be the last location");
			node.set_leaf(location[0], content);
		}
	}
	
	

	
	
	///
	///Traverse the tree and get the node at this location
	///
	///
	
	pub fn get(&self, location:&Vec<u64>)->&Option<T>{
		let mut m_location = location.clone();
		let voxtree = self.get_tree_recursive(&mut m_location);
		&voxtree.content
	}
	///
	///internal implementation of getting the Voxtree
	///
	fn get_tree_recursive(&self, location:&mut Vec<u64>)->&Voxtree<T>{
		let node = self.get_node(location[0]);
		if location.len() > 1 {
			location.remove(0);
			return node.get_tree_recursive(location);
		}
		else{
			assert!(location.len() == 1, "This should be the last location");
			return node;
		}
	}
	
	
	
	///A much better implementation of setting
	pub fn set_path(&mut self, location:u64){
		if self.is_empty(location){
			self.children.push(Voxtree::new());
		}
		self.bitset = self.bitset | location;
	}
	
	pub fn set_leaf(&mut self, location:u64, content:&mut Option<T>){
		self.bitset = self.bitset | location;
		self.content = content.take();
	}
	
	///unsetting bits, there should be a cleanup operation to unset nodes that has 0 value
	pub fn unset(&mut self, location:u64){
		let index = self.index_of(location);
		self.children.remove(index);
		self.bitset = self.bitset & !location;
	}

	///
	/// get the child node located  
	///
	fn get_node(&self, location:u64)->&Voxtree<T>{
		if self.is_occupied(location){
			let index = self.index_of(location);
			return &self.children[index];
		}
		else{
			panic!("No Voxtree at location: {:8b}",location);
		}
	}
	
	///
	/// Get the node as mutable at this location
	///
	///
	
	pub fn get_as_mut(&mut self, location:u64)->&mut Voxtree<T>{
		if self.is_occupied(location){
			let index = self.index_of(location);
			return &mut self.children[index];
		}
		else{
			panic!("No Voxtree at location: {:8b}",location);
		}
	}
	
	///
	/// checks whether this node is set or not
	///
	fn is_occupied(&self, location:u64)->bool{
		self.bitset & location == location
	}
	
	///
	/// checks whether the node has value or not
	///
	fn is_empty(&self, location:u64)->bool{
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
	fn index_of(&self, location:u64)->usize{
		let mut index = 0;
		for i in 0..constants::BITS{
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
	/*
	pub fn is_point_occupied(&self, lod:&LOD, x:i64, y:i64 ,z:i64)->bool{
		if location::is_bounded(lod, x, y, z){ //no more bounds check if the camera is located inside the one-world Voxtree
			let loc = location::from_xyz(lod, x as u64, y as u64, z as u64);
			return self.is_location_occupied(&loc);
		}
		false
	}
	*/	

	///
	/// check whether the a certain location is occupied or not, expressed in 
	/// location notation which is just an arrray of 8bit values which describes the location of the 
	/// voxel at each LOD (level of detail )
	///
	pub fn is_location_occupied(&self, location:&Vec<u64>)->bool{
		let mut m_location = location.clone();
		self.is_location_occupied_recursive(&mut m_location)
	}
	
	///
	/// private implementation, since location is mutated at each recursive pass, check whether the a certain location is occupied or not, expressed in 
	/// location notation which is just an arrray of 8bit values which describes the location of the 
	/// voxel at each LOD (level of detail )
	///
	fn is_location_occupied_recursive(&self, location:&mut Vec<u64>)->bool{
		if self.is_empty(location[0]){
			return false;
		}
		else{
			let node = self.get_node(location[0]);
			if location.len() > 1 {
				location.remove(0);
				return node.is_location_occupied_recursive(location);
			}
			else{// location.len() == 1
				assert!(location.len() == 1, "This should be the last location array");
				return node.is_occupied(location[0]);
			}
			panic!("Shouldn't reach here!");
		}
	}
	
	
	///
	/// return the total number of leaf nodes in the Voxtree
	//
	pub fn count_leaf(&self)->u64{
		self.count_leaf_internal(self)
	}
	
	/// 
	/// internal implementation of counting the leaf nodes in the Voxtree
	/// 
	fn count_leaf_internal(&self, node:&Voxtree<T>)->u64{
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
	fn count_nodes_internal(&self, node:&Voxtree<T>)->u64{
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
		count == constants::BITS
	}
	
	

}


impl <T: fmt::Display> fmt::Display for Voxtree<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(bitset: {:8b}), children: {}",self.bitset, self.children.len())
    }
}