extern crate balisong;
extern crate time;

use std::num::Float;
use std::num::SignedInt;
use std::sync::Arc;
use std::thread::Thread;
use std::old_io::File;
use time::PreciseTime;


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
use balisong::optimizer;
use balisong::voxelizer;
use balisong::model::Model;
use balisong::renderer;


fn main() {
	let lod = 13;
	let view_lod = lod;

	let limit = 1 << lod;
	let r = limit as u64 / 4 as u64;//TODO: cube does not work with limit/2 don't know why
	let cx = limit/2;
	let cy = limit/2;
	let cz = limit/2;
	let center = Point::new(cx, cy, cz);
	//let shape = Sphere::new(r, &center);
	let shape = Cube::new(r, &center);
	let shape_name = shape.name();
	println!("voxelizing...{}", shape_name);
	let start = PreciseTime::now();
	let mut root = voxelizer::voxelize(lod, shape);
	
	optimizer::optimize(&mut root,lod);//save memory
	
	let duration = start.to(PreciseTime::now());
	println!("Voxelizing took: {} seconds",duration.num_seconds());
	
	
	let view_limit = 1 << view_lod;
	let obj_scale = 1.0;
	
	let cam_loc = Point::new(-view_limit/2, -view_limit/2, -view_limit/2);
	//let cam_loc = Point::new(-view_limit, -view_limit, -view_limit);
	let pitch = (-45.0).to_radians();
	let yaw = (45.0).to_radians();
	let roll = 0.0;
	let camera = Camera::new(cam_loc.clone(), pitch, yaw, roll);
	//let screen = Screen::new(1920, 1080, 1920/2);
	let screen = Screen::new(800, 600, 800/2);
	
	let model = Model::new(Point::new(view_limit/2, view_limit/2, view_limit/2), root, obj_scale);
	let start = PreciseTime::now();
	println!("Rendering...");
	//let pixels = renderer::render_threaded(lod, view_lod, model, &screen, &camera);
	let pixels = renderer::render(lod, view_lod, model, &screen, &camera);
	
	let duration = start.to(PreciseTime::now());
	println!("Rendering took: {} seconds", duration.num_seconds());
	
	let filename = format!("./renders/{}lod{}view{}scale{}cam{}pitch{}yaw{}.ppm",
		shape_name, lod, view_lod, obj_scale, cam_loc, 
		pitch.to_degrees().round(), yaw.to_degrees().round());
	
	renderer::save_to_file(filename, pixels, screen.width, screen.height);
 }




