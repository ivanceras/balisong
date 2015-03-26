#Balisong

A voxel based renderer written in rust.


##Screenshots

######No Smoothing (Below)
![](https://raw.githubusercontent.com/ivanceras/balisong/master/screenshots/no_smoothing.png)



######Single Smoothing (Below)
![](https://raw.githubusercontent.com/ivanceras/balisong/master/screenshots/single_smooth.png)

Note: The purple dots indicated erroneous calculation upon averaging which have resulted to Normal(0, 0, 0)




######Dual smoothing / Final result
![](https://raw.githubusercontent.com/ivanceras/balisong/master/screenshots/dual_smooth.png)


##How to generate the image above
	
```
git clone https://github.com/ivanceras/balisong
cd balisong
cargo run --release --example render_solid_lucy
```

then look at `./renders` directory. File output is in `.ppm` format which should be viewable in linux

### A more complex scene

![](https://raw.githubusercontent.com/ivanceras/balisong/master/screenshots/complex.png)

```
cargo run --release --example render_complex
```

#Features
* No other library dependency except for standard rust library and extension library such as time and regex
* Normals are calculated based on voxel structure
* Written in rust :)


#Progress made since ivancerust
* This project is a progression of [ivancerust](https://github.com/ivanceras/ivancerust)
* Calculation of normals take a few minutes, and a few minutes more for smoothing the normals. This is however not yet optimized, and may improved in future iteration of this project
* This now uses sparse octree which speed up the searching of points to an average of 3 microsecond per pixel.
	* Rendering process still takes ~30 seconds, since there are only 8 CPU to parallelize the load of 2 million pixels (1920x1080)

* Normals are recalculated based on voxel structure, this is useful when you are procedurally generating terrains / models.
	

#Roadmap
* Make OpenCL graphics rendering pipeline. (https://github.com/luqmana/rust-opencl)
	* Why?, Because current rendering pipeline assumes the use of triangles, and rasterization
	* This project uses voxels and raytracing
		
* Lattice matrix transformation
* Particle System - uses grid voxel, rather than octree 
* Physics addition (https://github.com/sebcrozet/nphysics), voxel collision should be easy


##Previous discussion on reddit
http://www.reddit.com/r/programming/comments/2xnlv7/3d_voxel_renderer_using_raytracing_written_in_rust/



#Contributing
* This project is in need of an experienced OpenCL programmer to convert the rendering part of the code to OpenCL to make it work on the GPU. There is a lot of recursive calls though, but I will make an iterative version of those.
* If you are interested in making this a full blown renderer + physics engine, fork this project.

	
# For Updates
Follow me on twitter: [@ivanceras](https://twitter.com/ivanceras)
