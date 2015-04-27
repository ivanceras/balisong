/// voxstream will be used for fetching on demand additional details at a certain voxel location.
/// can also be used to contain material lookup such as color, specular, diffuse, refraction,reflection info.

use voxel::vox::Vox;
use lod::LOD;

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
	
	
	///determine the number of 1's at the higher LOD before it reaches at the required LOD
	///
	/// ie. location: 
	///    0001 0000
	///    0010 0000
	///    0000 1000
	///
	/// ie. contents ( each set bit points to the whole integer at the next level
	///		                          0:
	///                           [1101 1111]
	///
	///		    6:           5:          4:           3:          2:         1:          0:
	///		  [1001 0001] [1010 1010] [0101 0101] [0000 0001] [1111 000] [0000 1111] [0110 0111]
	///
	///      6:2         6:1      6:0
	///  [0101 0101]  [1010 0001] [0001 0001]  ....
	///
	///
	/// algorithmn : count the number of 1's at each level
	/// If how many 1's is there at the root node, that is equivalent to the number of bytes to be read at the succedding node
	/// The total of 1's that in the previous level is the total number of bytes to the next level and so on and so forth
	/// The only location that matters is the last location bitset, where it be the offset of the 1's
	
	pub fn total_ones(&self, required_lod: &LOD)->usize{
		let root = self.bitset[0].count_ones();
		root as usize
	}
	
	pub fn insert_root(&mut self){
		self.bitset.push(!0);
	}
	
	///
	///Traverse the tree and get the node at this location
	///
	///
	
	pub fn get_content(&self, location:&Vec<u64>)->&T{
		&self.content[0]
	}
}

