# Balisong

A voxel based raytracer written in rust.
Well, raycaster for now (no light bounce yet).


## Screenshots

#### Here is a golden statue

![](https://raw.githubusercontent.com/ivanceras/balisong/master/screenshots/complex_golden_smoothen.png)

```
cargo run --release --example render_complex10

```

then look at `./renders` directory. File output is in `.ppm` format which should be viewable in linux

Rendering took ~400 seconds on Intel Core i7, that is excluding the calculation and smoothing of normals.


The a voxel grid which is capable of holding (2^10)^3 = 1,073,741,824 voxels.
Of course, I stripped out the empty voxels and the completely occluded voxels
The gold color in hardcoded in the raytracer. So, everything will be golden.

Fully solid statue has 26,637,838 voxels (includes the occluded voxels, which is 2% of the total maximum grid space). Consumes a whooping 6.4 GiB of memory

The final carved out statue contains 2,102,146 surface voxels which is 8% of the total solid statue and 0.20% of the maximum space grid space.
It consumed only 700 MiB of memory which includes the calculated normals for each surface voxel.


# Features
* No other library dependency except for standard rust library and extension library such as time and regex
* Normals are calculated based on voxel structure
* Written in rust :)


# Progress
Daily Progress is logged in [Progress.md](https://github.com/ivanceras/balisong/blob/master/Progress.md)

# TODO's
Daily Todo's is logged in [TODO.md](https://github.com/ivanceras/balisong/blob/master/TODO.md)
	

# Roadmap
* Make OpenCL graphics rendering pipeline. (https://github.com/luqmana/rust-opencl)
	* Why?, Because current rendering pipeline assumes the use of triangles, and rasterization
	* This project uses voxels and raytracing
		
* Lattice matrix transformation
* Particle System - uses grid voxel, rather than octree 
* Physics addition (https://github.com/sebcrozet/nphysics), voxel collision should be easy


## Previous discussion on reddit
http://www.reddit.com/r/programming/comments/2xnlv7/3d_voxel_renderer_using_raytracing_written_in_rust/



# Contributing
* This project is in need of an experienced OpenCL programmer to convert the rendering part of the code to OpenCL to make it work on the GPU. There is a lot of recursive calls though, but I will make an iterative version of those.
* If you are interested in making this a full blown renderer + physics engine, fork this project.

	
# For Updates
Follow me on twitter: [@ivanceras](https://twitter.com/ivanceras)
