##March 13, 2015
	* Binvox loader works
	* Carving the voxels to exclude voxels that are completely included to save memory consumption.

##March 14, 2015
	* Converted implementation from morton sorted structure (searched using binary search) to Octrees.
	
	* Tracing is now done in threads, 1 thread per core delivers the maximum performance
	
##March 17,2015
	* Light calculation with normals now work
	
##March 24, 2015
	* Derivation of normals from voxel structure itself made progress, initial calculation. Looks ugly
	