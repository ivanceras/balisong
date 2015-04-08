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

##April 6, 2015
* Eliminated error calculation in normals which resulted in Normal(0,0,0) by substituting it with normals dervied from the center of the object

##April 8, 2015
* Reduced memory size from 6.4 GB(fully solid) to 5.3 GB(fully solid, ommited booleans) to 900 MB(carved out) to 713 MB(carved out normals only) by carving out completely occluded voxels. Using empty voxtree saves up to 1.1GB of unrequired contents
	* Fully solid (6.4 GB) - 26,637,838 solid voxels, occluded included  (2 % of the maximum space)
	* Carved out (720 MB)  - 2,102,146 surface voxels (8% of the total solid) (0.20 % of the maximum space)