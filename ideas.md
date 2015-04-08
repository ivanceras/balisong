For each light source, shoot rays out until it hits the voxel in the octree's the granularity (LOD) is 
dictated by the distance from the origin of the ray to the sub. The higher the LOD when the subject voxel is closer.

Situations where object are very far from the light source, but light source intensity is strong enough to reach the object at far location. The intensity of the light received by the object can just be stored in the parent and will be used by the child voxels. This follows the principle of LOD based on distance.

All light sources emit light rays at all direction (including spot lights). Spot light rays that supposed to hit the other side hits a very close surface (usually reflective material, then it is bounced at the opposite direction.

Occluded light source also follow this principle.

Loading LOD's of object at observer location will be traversed in spherical fashion.
Starting from the observer location, expand a sphere outward removing points as necessary when certain points hits a voxel.

voxels that are not completely opaque will have to continue traversing the ray until such threshold of opacity is reached.

##Voxelizing algorithmn
	* Taking the triangles mesh of a 3D model.
	* Decide which axis to slice the model from
	* Determine which triangles that touches the axis at a certain intercept.
		* Using a AARB box, if a triangle is intersected at this bounding box, then that triangle will be a part of the test.
http://fileadmin.cs.lth.se/cs/Personal/Tomas_Akenine-Moller/code/tribox3.txt
	* Determine the points that lie in the triangles.
	