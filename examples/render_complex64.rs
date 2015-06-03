extern crate balisong;
extern crate time;

use std::sync::Arc;
use std::thread::Thread;
use time::PreciseTime;
use std::f64::consts;


use balisong::ray::Ray;
use balisong::vector::Vector;
use balisong::point::Point;
use balisong::screen::Screen;
use balisong::color::Color;
use balisong::shape::Sphere;
use balisong::shape::Cube;
use balisong::shape::Shape;
use balisong::binvox::Binvox;
use balisong::camera::Camera;
use balisong::renderer;
use balisong::model::Model;
use balisong::voxelizer;
use balisong::lod::LOD;
use balisong::voxel::vox::Vox;


fn main() {
	let overall_start = PreciseTime::now();
	let (lod, normals) = Binvox::read_file(format!("data/complex_64.binvox"));
	normals.count_leaves();
	//normals.traverse();
	let screen = Screen::new(1920, 1080, 1920/2);

	
	let view_lod = screen.get_view_lod();
	//let view_lod = LOD::new(lod.lod-4);
	println!("view_lod: {}", view_lod);
	println!("lod: {}",lod);
	//let view_lod = LOD::new(4);
	let limit = lod.limit as i64;
	let r = limit as u64 / 4 as u64;
	let cx = limit/2;
	let cy = limit/2;
	let cz = limit/2;
	let center = Point::new(cx, cy, cz);

	let voxel_grid_size = limit * limit * limit;
	
	
	
	let view_limit = view_lod.limit as i64;
	let obj_scale = 1.0;
	
	let cam_loc = Point::new(0, -3*view_limit/4, view_limit/10);
	let pitch = (0.0f64).to_radians();
	let yaw = (0.0f64).to_radians();
	let roll = (0.0f64).to_radians();
	
	let camera = Camera::new(cam_loc.clone(), pitch, yaw, roll);
	
	let model = Model::new(Point::new(view_limit/2, view_limit/2, view_limit/2), normals, obj_scale );
	let start = PreciseTime::now();
	println!("Rendering...");
	//45 s to 9s
	let pixels = renderer::render_threaded(&lod, &view_lod, model, &screen, &camera);
	let duration = start.to(PreciseTime::now());
	println!("Rendering took: {} seconds", duration.num_seconds());
	
	let filename = format!("./renders/10complex{}_{}_{}_{}[{}_{}_{}].ppm",
		lod, view_lod, obj_scale, cam_loc, 
		pitch.to_degrees().round(), yaw.to_degrees().round(), roll.to_degrees().round());
	
	renderer::save_to_file(filename, pixels, screen.width, screen.height);
	let overall_duration = overall_start.to(PreciseTime::now());
	println!("Overall process took: {} sec", overall_duration.num_seconds());
 }

 
