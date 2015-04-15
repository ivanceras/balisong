extern crate time;

use voxtree::Voxtree;
use point::Point;
use shape::{Shape, Sphere};
use normal::Normal;
use std::option::Option;
use color::Color;
use screen::Screen;
use camera::Camera;
use time::PreciseTime;
use model::Model;
use lod::LOD;

mod voxtree;
mod point;
mod shape;
mod normal;
mod vector;
mod color;
mod location;
mod neighbors;
mod voxelizer;
mod camera;
mod screen;
mod ray;
mod raytracer;
mod model;
mod renderer;
mod constants;
mod lod;

fn main() {
	let lod = LOD::new(4);
	let screen = Screen::new(1920, 1080, 1920/2);
	let view_lod = screen.get_view_lod();
	println!("lod = {}",lod);
	println!("view_lod = {}",view_lod);

	let limit = lod.limit; 
	let r = limit as u64 / 4 as u64;
	let cx = (limit/2) as i64;
	let cy = (limit/2) as i64;
	let cz = (limit/2) as i64;
	let center = Point::new(cx, cy, cz);
	let shape = Sphere::new(r, &center);
	let shape_name = shape.name();
	println!("voxelizing...{}", shape_name);
	let start = PreciseTime::now();
	let normals = voxelizer::voxelize(&lod, shape);
	//voxelizer::calculate_normals(&root, lod);
	
	let duration = start.to(PreciseTime::now());
	println!("Voxelizing took: {} seconds",duration.num_seconds());
	let voxel_grid_size = limit * limit * limit;
	println!("voxel grid size: {}", voxel_grid_size);
	
	let view_limit = view_lod.limit as i64;
	let obj_scale = 1.0;
	
	//let cam_loc = Point::new(view_limit/2, -view_limit/2, view_limit/2);
	let cam_loc = Point::new(0, -view_limit, 0);
	let pitch = (0.0f64).to_radians();
	let yaw = (0.0f64).to_radians();
	let roll = (0.0f64).to_radians();
	let camera = Camera::new(cam_loc.clone(), pitch, yaw, roll);
	
	let start = PreciseTime::now();
	println!("Rendering...");
	
	let model = Model::new(Point::new(view_limit/2, view_limit/2, view_limit/2), normals, obj_scale);
	//let model = Model::new(Point::new(0, 0, 0), root, normals, obj_scale);
	
	let pixels = renderer::render_threaded(&lod, &view_lod, model, &screen, &camera);
	
	let duration = start.to(PreciseTime::now());
	println!("Rendering took: {} ms", duration.num_milliseconds());
	
	let filename = format!("./renders/{}lod{}view{}scale{}cam{}pitch{}yaw{}.ppm",
		shape_name, lod.lod, view_lod.lod, obj_scale, cam_loc, 
		pitch.to_degrees().round(), yaw.to_degrees().round());
	
	renderer::save_to_file(filename, pixels, screen.width, screen.height);
 }




