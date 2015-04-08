use std::fmt;
use std::num::Float;

use location;
use constants;
use lod::LOD;


pub struct EmptyVoxtree{
	pub bitset:u64,//contains the information of which of sector belongs to 
	pub children:Vec<EmptyVoxtree>,//leaf only when there is no children content, children.len() == 0,
}


impl EmptyVoxtree{
	
	pub fn new()->EmptyVoxtree{
		EmptyVoxtree{bitset:0, children:Vec::new()}
	}

	pub fn set_tree_non_recursive(&mut self, location:&Vec<u64>){
		let mut stack:Vec<&mut EmptyVoxtree> = Vec::new();
		stack.push(self);
		for i in 0..location.len() {
			let mut top:&mut EmptyVoxtree = match stack.pop(){
				Some(x) => x,
				None => panic!("Oh no's, stack in empty!"),
			};
			if top.is_empty(location[i]){
				top.set(location[i]);
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
			last_node.set(location[last]);
		}
		last_node.set(location[last]);
	}
	

	
	
	///
	///Traverse the tree and get the node at this location
	///
	///
	/*
	fn get_tree(&self, location:&Vec<u64>)->&EmptyVoxtree{
		let mut m_location = location.clone();
		self.get_tree_internal(&mut m_location)
	}
	*/
	
	///
	///internal implementation of getting the EmptyVoxtree
	///
	fn get_tree_internal(&self, location:&mut Vec<u64>)->&EmptyVoxtree{
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
	
	///A much better implementation of setting
	pub fn set(&mut self, location:u64){
		//println!("Taking the content to the T in set_ref... for location: {}",location);
		self.children.push(EmptyVoxtree::new());
		self.bitset = self.bitset | location;
	}
	
	///unsetting bits, there should be a cleanup operation to unset nodes that has 0 value
	pub fn unset(&mut self, location:u64){
		let index = self.index_of(location);
		self.children.remove(index);
		self.bitset = self.bitset | !location;
	}

	///
	/// get the child node located  
	///
	fn get_node(&self, location:u64)->&EmptyVoxtree{
		if self.is_occupied(location){
			let index = self.index_of(location);
			return &self.children[index];
		}
		else{
			panic!("No EmptyVoxtree at location: {:8b}",location);
		}
	}
	
	///
	/// Get the node as mutable at this location
	///
	///
	
	pub fn get_as_mut(&mut self, location:u64)->&mut EmptyVoxtree{
		if self.is_occupied(location){
			let index = self.index_of(location);
			return &mut self.children[index];
		}
		else{
			panic!("No EmptyVoxtree at location: {:8b}",location);
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
		if location::is_bounded(lod, x, y, z){ //no more bounds check if the camera is located inside the one-world EmptyVoxtree
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
	/// return the total number of leaf nodes in the EmptyVoxtree
	//
	pub fn count_leaf(&self)->u64{
		self.count_leaf_internal(self)
	}
	
	/// 
	/// internal implementation of counting the leaf nodes in the EmptyVoxtree
	/// 
	fn count_leaf_internal(&self, node:&EmptyVoxtree)->u64{
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
	fn count_nodes_internal(&self, node:&EmptyVoxtree)->u64{
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


impl fmt::Display for EmptyVoxtree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(bitset: {:8b}), children: {}",self.bitset, self.children.len())
    }
}