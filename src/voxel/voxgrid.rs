/// voxgrid will be used for particle simulation/rigid body motion animation, 
/// this requires a faster memory access and transformation.
/// Implemented with a series of 64 bits, the location of the bits determine the x,y,z location
use voxel::vox::Vox;
use lod::LOD;
use location;

pub struct Voxgrid<T>{
	lod:LOD,
	bitset:Vec<u64>,
	content:Vec<T>,
}

impl <T> Voxgrid<T>{
	fn new(lod:&LOD)->Self{
		Voxgrid{lod:lod.clone(), 
			bitset:Vec::new(), 
			content:Vec::new()
		}
	}
	
	fn get(&self, x:u64, y:u64, z:u64)->&T{
		let index = location::xyz_to_index(&self.lod, x,y,z);
		&self.content[index as usize]
	}
}

