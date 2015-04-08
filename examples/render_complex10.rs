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
use balisong::renderer;
use balisong::model::Model;
use balisong::voxelizer;


fn main() {
	let (lod, normals) = Binvox::read_file(format!("data/complex10.binvox"));
	
	let screen = Screen::new(1920, 1080, 1920/2);

	
	let view_lod = screen.get_view_lod();
	let limit = lod.limit as i64;
	let r = limit as u64 / 4 as u64;
	let cx = limit/2;
	let cy = limit/2;
	let cz = limit/2;
	let center = Point::new(cx, cy, cz);
	let start = PreciseTime::now();

	let duration = start.to(PreciseTime::now());

	let voxel_grid_size = limit * limit * limit;
	let total_nodes = normals.count_nodes();
	println!("There are {} total nodes", total_nodes);
	let empty = voxel_grid_size - total_nodes as i64;
	println!("empty: {} {}%", empty, (100.0 * empty as f64/voxel_grid_size as f64).round());
	println!("filled {} %", (100.0 * total_nodes as f64 / voxel_grid_size as f64).round());
	let leaf_nodes = normals.count_leaf();
	println!("There are {} leaf nodes {}%", leaf_nodes, (100.0 * leaf_nodes as f64 / total_nodes as f64).round());
	
	
	
	let view_limit = view_lod.limit as i64;
	let obj_scale = 1.0;
	
	let cam_loc = Point::new(0, -3*view_limit/4, view_limit/10);
	let pitch = to_radians(0.0);
	let yaw = to_radians(0.0);
	let roll = to_radians(0.0);
	
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
 }

 
fn to_radians(degree:f64)->f64{
	degree * consts::PI / 180.0
}