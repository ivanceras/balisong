##March 13, 2015
* Binvox loader works
* Carving the voxels to exclude voxels that are completely included to save memory consumption.

##March 14, 2015
* Converted implementation from morton sorted structure (searched using binary search) to Octrees.
* Tracing is now done in threads, 1 thread per core delivers the maximum performance

##March 17,2015
* Light calculation with normals now work

##March 18, 2015
* Change the xyz orientation, pattern to unreal, blender, by using Z as up (previously Y as up)	

##March 24, 2015
* Derivation of normals from voxel structure itself made progress, initial calculation. Looks ugly
* Made the generated normals look smooth already. Tried 1st pass and 2nd pass smoothing.

##March 30, 2015
* Implemented finding the neighbor voxel using towards center of the model.
	* This can be improved by using the local geometric center of the parent models

##April 3, 2015
* Updated code to remove rust deprecated modules such as old_io

##April 4, 2015
* Renamed octree to voxtree since the number of children is not limited to 8 anymore, 64 children is optimal option for optimizing memory usage.
* Added blending of colors and light intensity.
* Tried recalculating of normals only when the point is hit in the computation of rays