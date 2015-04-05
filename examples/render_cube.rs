extern crate balisong;
extern crate time;

use std::num::Float;
use std::num::SignedInt;
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
use balisong::voxelizer;
use balisong::model::Model;
use balisong::renderer;
use balisong::lod::LOD;


fn main() {
	let lod = LOD::new(4);
	let screen = Screen::new(1920, 1080, 1920/2);
	let view_lod = screen.get_view_lod();

	let limit = lod.limit as i64;
	let r = limit as u64 / 4 as u64;
	let cx = limit/2;
	let cy = limit/2;
	let cz = limit/2;
	let center = Point::new(cx, cy, cz);
	let shape = Cube::new(r, &center);
	let shape_name = shape.name();
	println!("voxelizing...{}", shape_name);
	let start = PreciseTime::now();
	let (mut root, normals) = voxelizer::voxelize(&lod, shape);
	
	let duration = start.to(PreciseTime::now());
	println!("Voxelizing took: {} seconds",duration.num_seconds());
	
	
	let view_limit = view_lod.limit as i64;
	let obj_scale = 1.0;
	
	let cam_loc = Point::new(-view_limit/2, -view_limit/2, -view_limit);
	//let cam_loc = Point::new(-view_limit, -view_limit, -view_limit);
	let pitch = to_radians(60.0);
	let yaw = to_radians(-60.0);
	let roll = to_radians(0.0);
	let camera = Camera::new(cam_loc.clone(), pitch, yaw, roll);
	
	let model = Model::new(Point::new(view_limit/2, view_limit/2, view_limit/2), root, normals, obj_scale);
	let start = PreciseTime::now();
	println!("Rendering...");

	let pixels = renderer::render_threaded(&lod, &view_lod, model, &screen, &camera);
	
	let duration = start.to(PreciseTime::now());
	println!("Rendering took: {} seconds", duration.num_milliseconds());
	
	let filename = format!("./renders/cube{}_{}_{}_{}[{}_{}_{}].ppm",
		lod, view_lod, obj_scale, cam_loc, 
		pitch.to_degrees().round(), yaw.to_degrees().round(), roll.to_degrees().round());

	
	renderer::save_to_file(filename, pixels, screen.width, screen.height);
 }


fn to_radians(degree:f64)->f64{
	degree * consts::PI / 180.0
}

