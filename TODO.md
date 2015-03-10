##Mar 10, 2015

	* Let the binvox loader works
	* Create a new voxel format (.svo)
		* 1 contains the structure, the other the material information
	* Sparse Octree optimizer
		* If the 8 children of the voxel contains the same material (Color, normal, specularity, refraction). Store this information to the parent. Recursive apply the same logic, bottom-up.
		