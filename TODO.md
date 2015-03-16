## Mar 10, 2015

	* Let the binvox loader works
	* Create a new voxel format (.svo)
		* 1 contains the structure, the other the material information
	* Sparse Octree optimizer
		* If the 8 children of the voxel contains the same material (Color, normal, specularity, refraction). Store this information to the parent. Recursive apply the same logic, bottom-up.
	
### Memory Optimization
	* Data can be optimized by separating the material structure from the Octree.
	* Octree only provides the relative location of voxels
	* Normals will be on separate file, will be stored linearly
	* Material can just be a lookup, stored with respect to octree structure

### File streaming
	* Create an array of lookup table, all the possible material used.
	* For each LOD, make a grid, which stores the material indexes, arranged in morton encode and compressed using RLE (run-length-encoding).
	* Normals are stored in this arrangement as well
	

## March 12, 2015

	* Try to trace each pixel using threads		
	
## March 16, 2015
	 
	 * Add normals, injecting luminance from light source
	 * Make an efficient conversion algorithmn for determining the next voxel to evaluate rather than relying on location::from_xyz(lod, x,y,z) which performs a lot of calculations
	 * Implement a one world octree, which all object that are to be viewed will be on the octree. The world octree contains the camera location. Based on the camera location objects at minimum required LOD, will be loaded to this octree. The closer it is to the camera, the more details will be fetched. The loaded octree will be merged to the one world octree. There will be merging implementation of the octree.

	 * Merge point, Scale, rotation, translation, transformation lattice