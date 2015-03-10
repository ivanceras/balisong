extern crate time;

use octree::Octree;
use point::Point;
use shape::{Shape, Sphere, Cube};
use normal::Normal;
use std::option::Option;
use voxel::Voxel;
use color::Color;
use std::num::Float;
use scene::Scene;
use screen::Screen;
use camera::Camera;
use std::old_io::File;
use time::PreciseTime;

mod octree;
mod point;
mod shape;
mod normal;
mod vector;
mod voxel;
mod color;
mod location;
mod voxelizer;
mod scene;
mod camera;
mod screen;
mod ray;
mod raytracer;
mod model;


fn main() {
	let lod = 7;
	let view_lod = 8;//11;

	let limit = 1 << lod;
	let r = limit as u64 / 4 as u64;//TODO: cube does not work with limit/2 don't know why
	let cx = limit/2;
	let cy = limit/2;
	let cz = limit/2;
	let center = Point::new(cx, cy, cz);
	let shape = Sphere::new(r, &center);
	//let shape = Cube::new(r, &center);
	let shape_name = shape.name();
	println!("voxelizing...{}", shape_name);
	let start = PreciseTime::now();
	let root = voxelizer::voxelize(lod, shape);
	let duration = start.to(PreciseTime::now());
	println!("Voxelizing took: {} seconds",duration.num_seconds());
	
	
	let mut scene = Scene::new(view_lod);
	let view_limit = 1 << view_lod;
	let obj_scale = 1.0;
	scene.add(view_limit/2, view_limit/2, view_limit/2, root, obj_scale);
	
	let cam_loc = Point::new(-view_limit/2, -view_limit/2, -view_limit/2);
	//let cam_loc = Point::new(-view_limit, -view_limit, -view_limit);
	let pitch = (-45.0).to_radians();
	let yaw = (45.0).to_radians();
	let roll = 0.0;
	let camera = Camera::new(cam_loc.clone(), pitch, yaw, roll);
	scene.set_camera(camera);
	//let screen = Screen::new(1920, 1080, 1920/2);
	let screen = Screen::new(800, 600, 800/2);
	
	let start = PreciseTime::now();
	println!("Rendering...");
	let pixels = scene.render(lod, &screen);
	let duration = start.to(PreciseTime::now());
	println!("Rendering took: {} seconds", duration.num_seconds());
	
	save_to_file(format!("./renders/{}lod{}view{}scale{}cam{}pitch{}yaw{}.ppm",shape_name, lod, view_lod, obj_scale, cam_loc, pitch.to_degrees().round(), yaw.to_degrees().round()), pixels, screen.width, screen.height);
 }


//save pixels to file
pub fn save_to_file(filename:String, pixels:Vec<Color>, width:i64, height:i64){
	let mut file = File::create(&Path::new(&filename));
	let header = String::from_str("P6\n# CREATOR: lee\n");
	let size = format!("{} {}\n255\n", width, height);

	let mut buffer = Vec::new();
    buffer.push_all(header.into_bytes().as_slice());
    buffer.push_all(size.into_bytes().as_slice());
    
	for p in 0..pixels.len() {
		buffer.push(pixels[p].r);
		buffer.push(pixels[p].g);
		buffer.push(pixels[p].b);
	}
	file.write_all(buffer.as_slice());
	println!("Saved to {}",&filename);
}

