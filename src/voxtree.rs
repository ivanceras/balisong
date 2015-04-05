use std::fmt;
use std::num::Float;

use location;
use constants;
use vector::Vector;
use lod::LOD;


//TODO: 
//memory optimization: check to see if all 8 children in a voxel has same materials, then simplify 
//by removing all children and store the information in the parent
//
pub struct Voxtree<T>{
	pub bitset:u64,//contains the information of which of octants belongs to 
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
	pub fn set_tree1(&mut self, location:&Vec<u64>, content:&mut Option<T>){
		let mut m_location = location.clone();
		self.set_tree_internal(&mut m_location, content)
	}
	
	
	///
	/// Internal implementation for setting the Voxtree
	///
	
	fn set_tree_internal(&mut self, location:&mut Vec<u64>, content:&mut Option<T>){
		if self.is_empty(location[0]){//if empty, create a node with None content
			self.set(location[0], &mut None);
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
	
	pub fn set_tree_non_recursive(&mut self, location:&Vec<u64>, content:&mut Option<T>){
		let mut stack:Vec<&mut Voxtree<T>> = Vec::new();
		stack.push(self);
		for i in 0..location.len() {
			let mut top:&mut Voxtree<T> = match stack.pop(){
				Some(x) => x,
				None => panic!("Oh no's, stack in empty!"),
			};
			if top.is_empty(location[i]){
				top.set(location[i], &mut None);
			}
			let node = top.get_as_mut(location[i]);
			stack.push(node);
		}
		let last_node = match stack.pop(){
			 Some(x) => x,
			 None => panic!("stack in empty!"),
		};
		let last = location.len() - 1;
		if last_node.is_empty(location[last]){
			last_node.set(location[last], &mut None);
		}
		last_node.set(location[last], content);
	}
	

	
	
	///
	///Traverse the tree and get the node at this location
	///
	///
	/*
	fn get_tree(&self, location:&Vec<u64>)->&Voxtree<T>{
		let mut m_location = location.clone();
		self.get_tree_internal(&mut m_location)
	}
	*/
	
	pub fn get(&self, location:&Vec<u64>)->&Option<T>{
		let mut m_location = location.clone();
		let Voxtree = self.get_tree_internal(&mut m_location);
		&Voxtree.content
	}
	///
	///internal implementation of getting the Voxtree
	///
	fn get_tree_internal(&self, location:&mut Vec<u64>)->&Voxtree<T>{
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
	/*
	pub fn set(&mut self, location:u8, content:Option<T>){
		self.children.push(Voxtree::new());
		self.bitset = self.bitset | location;
		self.content = content;
	}
	*/
	///A much better implementation of setting
	pub fn set(&mut self, location:u64, content:&mut Option<T>){
		//println!("Taking the content to the T in set_ref... for location: {}",location);
		self.children.push(Voxtree::new());
		self.bitset = self.bitset | location;
		self.content = content.take();
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
	pub fn is_point_occupied(&self, lod:&LOD, x:i64, y:i64 ,z:i64)->bool{
		if location::is_bounded(lod, x, y, z){ //no more bounds check if the camera is located inside the one-world Voxtree
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
	pub fn is_location_occupied(&self, location:&Vec<u64>)->bool{
		let mut m_location = location.clone();
		self.is_location_occupied_internal(&mut m_location)
	}
	
	///
	/// private implementation, since location is mutated at each recursive pass, check whether the a certain location is occupied or not, expressed in 
	/// location notation which is just an arrray of 8bit values which describes the location of the 
	/// voxel at each LOD (level of detail )
	///
	fn is_location_occupied_internal(&self, location:&mut Vec<u64>)->bool{
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
	
	
	
	//at this location get the next voxel value
	fn next(&self, direction:Vector, length:u64){
		let dir = direction.scale(length as f64);
		let x = dir.x.round();
		let y = dir.y.round();
		let z = dir.z.round();
		//use this x,y,z as offset from the current location
	}

}


impl <T: fmt::Display> fmt::Display for Voxtree<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(bitset: {:8b}), children: {}",self.bitset, self.children.len())
    }
}