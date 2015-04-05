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
use balisong::renderer;
use balisong::model::Model;


fn main() {
	let (lod, mut root, normals) = Binvox::read_file(format!("data/city3.binvox"));
	
	let screen = Screen::new(1920, 1080, 1920/2);
	//let screen = Screen::new(800, 600, 800/2);

	
	let view_lod = screen.get_view_lod();//10;--better render with same LOD
	let limit = 1 << lod;
	let r = limit as u64 / 4 as u64;//TODO: cube does not work with limit/2 don't know why
	let cx = limit/2;
	let cy = limit/2;
	let cz = limit/2;
	let center = Point::new(cx, cy, cz);
	let start = PreciseTime::now();

	let duration = start.to(PreciseTime::now());

	let voxel_grid_size = limit * limit * limit;
	let total_nodes = root.count_nodes();
	println!("There are {} total nodes", total_nodes);
	let empty = voxel_grid_size - total_nodes as i64;
	println!("empty: {} {}%", empty, (100.0 * empty as f64/voxel_grid_size as f64).round());
	println!("filled {} %", (100.0 * total_nodes as f64 / voxel_grid_size as f64).round());
	let leaf_nodes = root.count_leaf();
	println!("There are {} leaf nodes {}%", leaf_nodes, (100.0 * leaf_nodes as f64 / total_nodes as f64).round());
	
	
	
	let view_limit = 1 << view_lod;
	let obj_scale = 0.5;
	
	let cam_loc = Point::new(-view_limit/2, -view_limit/2, view_limit/2);
	//let cam_loc = Point::new(0, 0, 0);
	let pitch = (-30.0).to_radians();
	let yaw = (30.0).to_radians();
	let roll = (0.0).to_radians();
	
	let camera = Camera::new(cam_loc.clone(), pitch, yaw, roll);
	
	let model = Model::new(Point::new(view_limit/2, view_limit/2, view_limit/2), root, normals, obj_scale );
	let start = PreciseTime::now();
	println!("Rendering...");
	//45 s to 9s
	let pixels = renderer::render_threaded(lod, view_lod, model, &screen, &camera);
	let duration = start.to(PreciseTime::now());
	println!("Rendering took: {} seconds", duration.num_seconds());
	
	let filename = format!("./renders/rough_city{}_{}_{}_{}[{}_{}_{}].ppm",
		lod, view_lod, obj_scale, cam_loc, 
		pitch.to_degrees().round(), yaw.to_degrees().round(), roll.to_degrees().round());
	
	renderer::save_to_file(filename, pixels, screen.width, screen.height);
 }
