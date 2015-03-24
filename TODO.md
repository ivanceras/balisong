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
	* Objects can have same material but of different colors (i.e stained glass, erroded concrete, painted plates) or any discolored materials due to environmental factors. Technically, the discoloration are just different atoms attached to the bigger material, but are thier mass is negligible to be considered as another set of material
	

## March 12, 2015

	* Try to trace each pixel using threads		
	
## March 16, 2015
	 
	 * Add normals, injecting luminance from light source
	 * Make an efficient conversion algorithmn for determining the next voxel to evaluate rather than relying on location::from_xyz(lod, x,y,z) which performs a lot of calculations
	 * Implement a one world octree, which all object that are to be viewed will be on the octree. The world octree contains the camera location. Based on the camera location objects at minimum required LOD, will be loaded to this octree. The closer it is to the camera, the more details will be fetched. The loaded octree will be merged to the one world octree. There will be merging implementation of the octree.

	 * Merge point, Scale, rotation, translation, transformation lattice
	 
## March 17, 2015
	
	* Implement a non-recursive function for get_tree in octree module, this will be translated to opencl version to allow real time rendering
	* Implement an own ply to voxel + normal + color converter
	* Implement a function to generate a normal of a voxel based on the neighbors
		* Get all the neighbors for this voxel, then take 3 at a time, generate normals then get the average of the normals
		* Neighbors of 0 is 1,2,3,4,5,6,7 at the same parent.
		* while the children (4,5,6,7) of the neighbor parent can also be a neighbor of this 0, but not (0,1,2,3) 
	
	
## March 19, 2015
	* location module needs to have "common_parent" function which tells what is the location of the common parent of two locations, This can be done by comparing at which part of the location array it starts to differ the values. Then we can then ignore the parent location path in the array, and do the calculations at the local level 
	
## March 23, 2015
    * Create normals based on voxel formation
        * Each voxel has 6 face neighbors, 12 side neighbors and 8 edge neighbors, at total of 26 neighbors. Calculated as (3^3-1)
        * Getting the neighboring voxel at 0,0,0

            ================
             6 face neighbors          
            ================
             0  0  1
             0  1  0
             1  0  0
             0  0 -1
             0 -1  0
            -1  0  0
              
             
            ================
             8 edges         
            ================        	
            -1 -1 -1 
            -1 -1  1
            -1  1 -1
            -1  1  1
             1 -1 -1
             1 -1  1
             1  1 -1	
             1  1  1 
             
             ================
             12 side neighbors       
             ================   
             
             0  1  1
             1  0  1
             1  1  0
             
             0 -1 -1
            -1  0 -1
            -1 -1  0


            -1  1  0
             0 -1  1
             0  1 -1

             1  0  1
             1 -1  0
             1  0 -1
         
##March 24, 2015
	* Calculation of normals on a different approach
		* Calculate the normals based on empty sides of the voxel data
		* If there is no occluded neighbor, we can use the empty voxel away from  the center as the point of reference
		* For all empty voxels (may exclude those which are close to non-empty) get the vector to this empty voxels, then get the average. It will then be use to approximate the normal
		* Do an averaging of neighboring voxel to smoothen the normal distribution
		* The holes is caused mainly of occluded points that is somehow hit on the ray traced, not sure whether a neighbor algorithm octree bug or raytracing bug