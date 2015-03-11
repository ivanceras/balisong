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
use balisong::scene::Scene;
use balisong::camera::Camera;
use balisong::optimizer;

fn main() {
	let (lod, mut root) = Binvox::read_file("data/heptoroid10.binvox");
	optimizer::optimize(&mut root, lod);
	let view_lod = lod;//10;--better render with same LOD
	let limit = 1 << lod;
	let r = limit as u64 / 4 as u64;//TODO: cube does not work with limit/2 don't know why
	let cx = limit/2;
	let cy = limit/2;
	let cz = limit/2;
	let center = Point::new(cx, cy, cz);
	let start = PreciseTime::now();

	let duration = start.to(PreciseTime::now());
	
	
	let mut scene = Scene::new(view_lod);
	let view_limit = 1 << view_lod;
	let obj_scale = 1.0;
	scene.add(view_limit/2, view_limit/2, view_limit/2, root, obj_scale);
	
	let cam_loc = Point::new(view_limit/4, view_limit/4, view_limit/4);
	let pitch = (90.0).to_radians();
	let yaw = (-45.0).to_radians();
	let roll = (0.0).to_radians();
	
	let camera = Camera::new(cam_loc.clone(), pitch, yaw, roll);
	scene.set_camera(camera);
	//let screen = Screen::new(1920, 1080, 1920/2);
	let screen = Screen::new(800, 600, 800/2);
	
	let start = PreciseTime::now();
	println!("Rendering...");
	let pixels = scene.render(lod, &screen);
	let duration = start.to(PreciseTime::now());
	println!("Rendering took: {} seconds", duration.num_seconds());
	
	let filename = format!("./renders/heptoroid_{}_{}_{}_{}[{}_{}_{}].ppm",
		lod, view_lod, obj_scale, cam_loc, 
		pitch.to_degrees().round(), yaw.to_degrees().round(), roll.to_degrees().round());
	
	scene.save_to_file(filename, pixels, screen.width, screen.height);
 }
