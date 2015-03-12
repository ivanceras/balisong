use std::old_io::File;
use std::sync::mpsc;
use std::num::Float;
use std::thread::Thread;
use std::sync::Arc;

use camera::Camera;
use screen::Screen;
use ray::Ray;
use raytracer;
use model::Model;
use color::Color;
use std::os;

//non threaded
pub fn render(lod:u8, view_lod:u8, model:Model, screen:&Screen, camera:&Camera)->Vec<Color>{
	println!("NO threads...");
	let mut pixels = Vec::new();
	let view_limit = 1 << view_lod;
	let max_distance = 2 * (view_limit as f64 * view_limit as f64).sqrt().round() as u64;
	let mut percentage = 0;
	for y in 0..screen.height{
		let new_percentage = (y as f64 * 100.0 / screen.height as f64).round() as u64;
		if new_percentage > percentage{
			println!("{}%",new_percentage);
		}
		percentage = new_percentage;
		for x in 0..screen.width{
			let color = trace_pixel(lod, view_lod, &model, screen, camera, x, y, max_distance);
			pixels.push(color);
		}	
	}
	pixels
}
	
/*
pub fn render_threaded(lod:u8, view_lod:u8, model:Model, screen:&Screen, camera:&Camera)->Vec<Color>{
    println!("With threads...");
    let mut pixels:Vec<Color> = Vec::new();
    let total = screen.width * screen.height;
	for t in range(0, total){
	    pixels.push(Color::black());
	}
	
	let (tx, rx) = mpsc::channel();
	
	let view_limit = 1 << view_lod;
	let max_distance = 2 * (view_limit as f64 * view_limit as f64).sqrt().round() as u64;
	let mut percentage = 0;
	

	let arc_model = Arc::new(model);
	let camera_clone = camera.clone();
	let arc_camera = Arc::new(camera_clone);
	let screen_clone = screen.clone();
	let arc_screen = Arc::new(screen_clone);
	
	for y in 0..screen.height{
		let new_percentage = (y as f64 * 100.0 / screen.height as f64).round() as u64;
		if new_percentage > percentage{
			println!("{}%",new_percentage);
		}
		percentage = new_percentage;
		for x in 0..screen.width{
			let tx = tx.clone();
			let arc_model_clone = arc_model.clone();
			let arc_camera_clone = arc_camera.clone();
			let arc_screen_clone = arc_screen.clone();
			Thread::spawn(move || {
				let color = trace_pixel(lod, view_lod, &arc_model_clone, &arc_screen_clone, &arc_camera_clone, x, y, max_distance);
				tx.send((x, y, color));
			});
		}	
	}
	
	for j in 0..screen.height{
	    for i in range(0, screen.width){
	    	let (x ,y ,color) = rx.recv().ok().expect("Could not recieve answer");
	   		let index = y * screen.width + x;
	   		pixels[index as usize] = color;
	    }
	}
	
	pixels
}
*/

pub fn render_threaded(lod:u8, view_lod:u8, model:Model, screen:&Screen, camera:&Camera)->Vec<Color>{
    println!("With threads...");
    println!("std::os::num_cpu {}",os::num_cpus());
    let mut pixels:Vec<Color> = Vec::new();
    let total = screen.width * screen.height;
	for t in 0..total{
	    pixels.push(Color::black());
	}
	
	let (tx, rx) = mpsc::channel();
	
	let view_limit = 1 << view_lod;
	let max_distance = 2 * (view_limit as f64 * view_limit as f64).sqrt().round() as u64;
	let mut percentage = 0;
	

	let arc_model = Arc::new(model);
	let camera_clone = camera.clone();
	let arc_camera = Arc::new(camera_clone);
	let screen_clone = screen.clone();
	let arc_screen = Arc::new(screen_clone);
	
	let width = screen.width;
	
	for y in 0..screen.height{
		let new_percentage = (y as f64 * 100.0 / screen.height as f64).round() as u64;
		if new_percentage > percentage{
			println!("{}%",new_percentage);
		}
		percentage = new_percentage;
		let tx = tx.clone();
		let arc_model_clone = arc_model.clone();
		let arc_camera_clone = arc_camera.clone();
		let arc_screen_clone = arc_screen.clone();
		Thread::spawn(move || {
			let mut line = Vec::new();
			for x in 0..width{
					let color = trace_pixel(lod, view_lod, &arc_model_clone, &arc_screen_clone, &arc_camera_clone, x, y, max_distance);
					line.push(color);
			}	
			tx.send((y, line));
		});
	}
	
	for j in 0..screen.height{
    	let (y, line) = rx.recv().ok().expect("Could not recieve answer");
   		for i in 0..screen.width{
   			let index = y * screen.width + i;
			pixels[index as usize] = line[i as usize].clone();
		}
	}
	
	pixels
}
	
pub fn trace_pixel(lod:u8, view_lod:u8, model:&Model, screen:&Screen, camera:&Camera, x:i64, y:i64, max_distance:u64)->Color{
	let pixel_vector = screen.at_pixel(x, y);
	let pixelv_yaw_rotated = pixel_vector.rotate_at_y(camera.yaw);
	let pixelv_pitch_rotated = pixelv_yaw_rotated.rotate_at_x(camera.pitch);
	let pixelv_roll_rotated = pixelv_pitch_rotated.rotate_at_z(camera.roll);
	let pixel_ray = Ray::new(&camera.location, pixelv_roll_rotated);
	let color = raytracer::trace_ray(lod, view_lod, pixel_ray, model, model.scale, max_distance);
	color
}


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

