///
/// a streaming interface to stream in voxel structure and materials
///

pub VoxelStream{
	svo_stream:Vec<u8>,
	material_lut:Vec<Material>,
}

impl VoxelStream{
	
	//load svo, normals, materials
	//load("lucy", 8);
	pub fn load(filename:String, lod:u8){
		
	}
	
	//load_svo("lucy.svo",8);
	pub fn load_svo(filename:String, lod:u8){
		Octree::new()
	}
	
	//load_normals("lucy.norm",8);
	pub fn load_normals(filename:String, lod:u8){
		
	}
	
	//load_materials("lucy.mat")
	pub fn load_materials(filename:String, lod:u8){
		
	}
	
	//get the normal at this location
	pub fn get_normal(location:&Vec<u8>)->Normal{
		
	}
	
	//get the material at this location
	pub fn get_material(location:&Vec<u8>)->Material{
		
	}
	
	//at each level, write the octree value
	pub fn write_svo(filename:String, octree:Octree){
		
	}

	pub fn write_normals(filename:String, octree:Octree){
		
	}
}