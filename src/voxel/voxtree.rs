use voxel::vox::Vox;

/// voxel tree for fast traversal of content
#[derive(Clone)]
pub struct Voxtree<T>{
	pub bitset:u64,
	pub content:Option<T>,
	pub children:Vec<Voxtree<T>>,
}

impl <T> Voxtree<T>{
	
	pub fn new()->Self{
		Voxtree{bitset:0u64, content: None, children:Vec::new()}
	}
	
	///
	///Traverse the tree and get the node at this location
	///
	///
	
	pub fn get_content(&self, location:&Vec<u64>)->&Option<T>{
		let voxtree = self.get_tree(location);
		&voxtree.content
	}
	
	pub fn set_content(&mut self, location:&Vec<u64>, content:&mut Option<T>){
		let mut stack = Vec::new();
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
	
	pub fn set_leaf(&mut self, location:u64, content:&mut Option<T>){
		self.bitset = self.bitset | location;
		self.content = content.take();
	}
	

	
}

impl <T> Vox for Voxtree<T>{
	
	fn bitset(&self)->u64{
		self.bitset
	}
	
	fn children(&self, index:usize)->&Self{
		&self.children[index]
	}
	
	fn mut_children(&mut self, index:usize)->&mut Self{
		&mut self.children[index]
	}
	
	fn num_children(&self)->usize{
		self.children.len()
	}
	
	fn new_children(&mut self){
		self.children.push(Voxtree::new());
	}
	
	fn or_bitset(&mut self, location:u64){
		self.bitset = self.bitset | location;
	}
	
	fn drain(&mut self){
		drop(self);
	}
}
