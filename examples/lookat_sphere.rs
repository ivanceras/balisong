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
use balisong::voxelizer;
use balisong::model::Model;
use balisong::renderer;


fn main() {
	let lod = 10;
	
	let screen = Screen::new(800, 600, 800/2);
	//let view_lod = lod;
	let view_lod = screen.get_view_lod();

	let limit = 1 << lod;
	let r = limit as u64 / 4 as u64;//TODO: cube does not work with limit/2 don't know why
	let cx = limit/2;
	let cy = limit/2;
	let cz = limit/2;
	let center = Point::new(cx, cy, cz);
	let shape = Sphere::new(r, &center);
	let shape_name = shape.name();
	println!("voxelizing...{}", shape_name);
	let start = PreciseTime::now();
	let (mut root, normals) = voxelizer::voxelize(lod, shape);
	
	let duration = start.to(PreciseTime::now());
	println!("Voxelizing took: {} seconds",duration.num_seconds());
	let voxel_grid_size = limit * limit * limit;
	println!("voxel grid size: {}", voxel_grid_size);
	let total_nodes = root.count_nodes();
	println!("There are {} total nodes", total_nodes);
	let empty = voxel_grid_size - total_nodes as i64;
	println!("empty: {} {}%", empty, (100.0 * empty as f64/voxel_grid_size as f64).round());
	println!("filled {} %", (100.0 * total_nodes as f64 / voxel_grid_size as f64).round());
	let leaf_nodes = root.count_leaf();
	println!("There are {} leaf nodes {}%", leaf_nodes, (100.0 * leaf_nodes as f64 / total_nodes as f64).round());
	
	let view_limit = 1 << view_lod;
	let obj_scale = 1.0;
	
	//let cam_loc = Point::new(view_limit/2, -view_limit/2, view_limit/2);
	let cam_loc = Point::new(0, -view_limit/2, 0);
	let pitch = (0.0).to_radians();
	let yaw = (0.0).to_radians();
	let roll = 0.0;
	let mut camera = Camera::new(cam_loc.clone(), pitch, yaw, roll);
	//camera.look_at(&Point::new(view_limit/2, view_limit/2, view_limit/2));
	//camera.look_at(&Point::new(view_limit, view_limit, view_limit));
	camera.look_at(&Point::new(0, 0, 0));

	let start = PreciseTime::now();
	println!("Rendering...");
	
	let model = Model::new(Point::new(view_limit/2, view_limit/2, view_limit/2), root, normals, obj_scale);
	
	let pixels = renderer::render_threaded(lod, view_lod, model, &screen, &camera);
	
	let duration = start.to(PreciseTime::now());
	println!("Rendering took: {} ms", duration.num_milliseconds());
	
	let filename = format!("./renders/lookat{}lod{}view{}scale{}cam{}pitch{}yaw{}.ppm",
		shape_name, lod, view_lod, obj_scale, cam_loc, 
		camera.pitch.to_degrees().round(), camera.yaw.to_degrees().round());
	
	renderer::save_to_file(filename, pixels, screen.width, screen.height);
 }




