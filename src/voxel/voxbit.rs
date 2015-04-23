///
/// voxel structure, a simple voxel structure that only describes the structure of the voxel at different LOD. 
/// Information such as normals, colors, specularity are not present in the structure.(voxlinear)
/// This has no content

use voxel::vox::Vox;

pub struct Voxbit{
	pub bitset:u64,
	pub children:Vec<Voxbit>
}

impl Voxbit{
	
	pub fn new()->Self{
		Voxbit{bitset:0u64, children:Vec::new()}
	}
	
	pub fn set_location(&mut self, location:&Vec<u64>){
		let mut stack = Vec::new();
		stack.push(self);
		for i in 0..location.len() {
			let mut top = match stack.pop(){
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
	}
	

}

impl Vox for Voxbit{
	
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
		self.children.push(Voxbit::new());
	}
	
	fn or_bitset(&mut self, location:u64){
		self.bitset = self.bitset | location;
	}
	fn drain(&mut self){
		drop(self);
	}
}
