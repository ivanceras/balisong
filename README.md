#Balisong

A voxel based renderer written in rust.


##Screenshots
	
Final result (dual smoothing)
![](https://raw.githubusercontent.com/ivanceras/balisong/master/screenshots/dual_smooth.png)

Single Smoothing

![](https://raw.githubusercontent.com/ivanceras/balisong/master/screenshots/single_smooth.png)

No Smoothing

![](https://raw.githubusercontent.com/ivanceras/balisong/master/screenshots/no_smoothing.png)

##How to generate the image above
	
	```
	cargo run --release --example render_solid_lucy
	
	```



#Features
	* No other library dependency except for standard rust library and extension library such as time and regex
	* Normals are calculated based on voxel structure


#Progress made since ivancerust
	* This project is a progression of ivancerust (which i didn't remove, coz I want to create progress documents, which can be useful for those who want to learn on how things are derived)
	* This now uses sparse octree which speed up the searching of points to and average of 3 microsecond per pixel.
		* Rendering process still takes ~30 seconds, since there are only 8 CPU to parallelize the load of 2 million pixels (1920x1080)
	
	* Normals are recalculated based on voxel structure, this is useful when you are procedurally generating terrains / models.
	

#Roadmap
	* Make OpenCL graphics rendering pipeline. (https://github.com/luqmana/rust-opencl)
		* Why?, Because current rendering pipeline assumes the use of triangles, and rasterization
		* This project uses voxels and raytracing
			
	* Lattice matrix transformation
	* Particle System
	* physics addition (https://github.com/sebcrozet/nphysics)


##Previous discussion on reddit
	http://www.reddit.com/r/programming/comments/2xnlv7/3d_voxel_renderer_using_raytracing_written_in_rust/



#Contributing
	* This project need an experienced OpenCL programmer to convert the rendering part of the code to OpenCL to make it work on the GPU. There is a lot of recursive calls though, but I will make an iterative version of those.
	* If you are interested in making this a full blown renderer + physics engine, fork this project.

	
# Updates
	Follow me on twitter: @ivanceras