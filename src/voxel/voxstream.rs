/// voxstream will be used for fetching on demand additional details at a certain voxel location.
/// can also be used to contain material lookup such as color, specular, diffuse, refraction,reflection info.

use voxel::vox::Vox;

pub struct Voxstream<T>{
	bitset:Vec<u64>,
	content:Vec<T>,
}

impl <T> Voxstream<T>{
	
	pub fn new()->Self{
		Voxstream{bitset:Vec::new(), content:Vec::new()}
	}	
	
	///traverse through the stream while counting the 1's that is encountered
	fn get_tree(&self, location:&Vec<u64>)->&Self{
		panic!("Not yet implemented");
	}
	
	pub fn get_index(&self, location:&Vec<u64>)->usize{
		for loc in location{
			println!("{}",loc);
		}
		0
	}
	
	///
	///Traverse the tree and get the node at this location
	///
	///
	
	pub fn get_content(&self, location:&Vec<u64>)->&T{
		&self.content[0]
	}
}

