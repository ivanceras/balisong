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

	pub fn set_tree_iterative(&mut self, location:&Vec<u64>, content:&mut Option<T>){
		let mut stack:Vec<&mut Voxtree<T>> = Vec::new();
		stack.push(self);
		for i in 0..location.len() {
			let mut top:&mut Voxtree<T> = match stack.pop(){
				Some(x) => x,
				None => panic!("Oh no's, stack in empty!"),
			};
			top.set_path(location[i]);
			let node = top.get_as_mut(location[i]);
			stack.push(node);
		}
		let last_node = match stack.pop(){
			 Some(x) => x,
			 None => panic!("stack in empty!"),
		};
		let last = location.len() - 1;
		last_node.set_path(location[last]);
		last_node.set_leaf(location[last], content);
	}
	

	
	
	///
	///Traverse the tree and get the node at this location
	///
	///
	
	pub fn get(&self, location:&Vec<u64>)->&Option<T>{
		let voxtree = self.get_tree_iterative(location);
		&voxtree.content
	}
	
	pub fn get_tree_iterative(&self, location:&Vec<u64>)->&Voxtree<T>{
		let mut stack:Vec<&Voxtree<T>> = Vec::new();
		stack.push(self);
		let last = location.len() - 1;
		for i in 0..last{
			let top:&Voxtree<T> = match stack.pop(){
				Some(x) => x,
				None => panic!("Oh no's, stack in empty!"),
			};
			let node = top.get_node(location[i]);
			stack.push(node);
		}
		let last_node = match stack.pop(){
			 Some(x) => x,
			 None => panic!("stack in empty!"),
		};
		last_node.get_node(location[last])
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
			let fast_index = self.fast_index_of(location);
			assert!(index == fast_index, "fast index should yield the same");
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
			let fast_index = self.fast_index_of(location);
			assert!(index == fast_index, "fast index should yield the same");
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
	/// get the actual index of the child from the vector base on bitset value, 
	/// by which the actual vector array is sparsed
	/// logic: count the number of 1's of bitset before the first and only 1 in location
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
	///short method
	/// 
	fn fast_index_of(&self, location:u64)->usize{
		let location = location - 1;
		let ones = self.bitset & location;
		ones.count_ones() as usize
	}
	
	pub fn is_location_occupied_iterative(&self, location:&Vec<u64>)->(usize, bool){
		let mut stack = Vec::new();
		stack.push(self);
		let last = location.len() - 1;
		for i in 0..location.len(){
			let top = match stack.pop(){
				Some(x) => x,
				None => panic!("Empty stack!"),
			};
			if top.is_empty(location[i]){
				return (i, false);
			}
			let node = top.get_node(location[i]);
			stack.push(node);
		}
		let last_node = match stack.pop(){
			Some(x) => x,
			None => panic!("Empty stack!"),
		};
		(last, last_node.is_occupied(location[last]))
	}
	

}


impl <T: fmt::Display> fmt::Display for Voxtree<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(bitset: {:8b}), children: {}",self.bitset, self.children.len())
    }
}