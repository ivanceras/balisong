use std::old_io::File;
use std::sync::mpsc;
use std::num::Float;
use std::thread::Thread;
use std::sync::Arc;
use time::PreciseTime;

use camera::Camera;
use screen::Screen;
use ray::Ray;
use raytracer;
use model::Model;
use color::Color;
use std::os;

///non threaded
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


///
/// divide to the number of CPU
///
pub fn render_threaded(lod:u8, view_lod:u8, model:Model, screen:&Screen, camera:&Camera)->Vec<Color>{
    println!("With threads...");
    println!("number of CPU {}",os::num_cpus());
    let cores = os::num_cpus();
    let mut pixels:Vec<Color> = Vec::new();
    let total = (screen.width * screen.height) as usize;
	for t in 0..total{
	    pixels.push(Color::black());
	}
	
	let (tx, rx) = mpsc::channel();
	
	let view_limit = 1 << view_lod;
	let max_distance = 2 * (view_limit as f64 * view_limit as f64).sqrt().round() as u64;
	

	let arc_model = Arc::new(model);
	let camera_clone = camera.clone();
	let arc_camera = Arc::new(camera_clone);
	let screen_clone = screen.clone();
	let arc_screen = Arc::new(screen_clone);
	
	let width = screen.width;
	let parts = (total as f64 / cores as f64).ceil() as usize;
	for i in 0..cores{
		let tx = tx.clone();
		let arc_model_clone = arc_model.clone();
		let arc_camera_clone = arc_camera.clone();
		let arc_screen_clone = arc_screen.clone();
		let start = i * parts;
		let end = (i+1) * parts;
		println!("Spawning {} to {}", start, end);
		Thread::spawn(move || {
			let mut line = Vec::new();
			let mut durations = Vec::new();
			for index in start..end{
				if index > total{
					break;
				}
				let y = index as i64 / width;
				let x = index as i64 - (y * width);
				let start = PreciseTime::now();
				let color = trace_pixel(lod, view_lod, &arc_model_clone, &arc_screen_clone, &arc_camera_clone, x, y, max_distance);
				let duration = start.to(PreciseTime::now());
				durations.push(duration.num_milliseconds() as f64);
				line.push(color);
			}	
			tx.send((start, end, line, durations));
		});
	}
	
	let mut all_average = Vec::new();
	for j in 0..cores{
		let new_percentage = (j as f64 * 100.0 / cores as f64).round() as u64;
    	let (start, end, line, durations) = rx.recv().ok().expect("Could not recieve answer");
    	let per_pixel_ave = get_average(&durations);
    	all_average.push(per_pixel_ave);
		println!("{}% core[{}] per_pixel_ave: {} milliseconds", new_percentage, j, per_pixel_ave);
   		let mut cnt = 0;
   		for i in start..end{
			pixels[i] = line[cnt].clone();
			cnt += 1;
		}
	}
	println!("Overall pixel average duration: {} ms ",get_average(&all_average));
	
	pixels
}


fn get_average(durations:&Vec<f64>)->f64{
	let mut total_duration = 0f64;
	for i in 0..durations.len(){
		total_duration += durations[i as usize] as f64;
	}
	total_duration as f64 / durations.len() as f64
}

/*	
pub fn trace_pixel(lod:u8, view_lod:u8, model:&Model, screen:&Screen, camera:&Camera, x:i64, y:i64, max_distance:u64)->Color{
	let pixel_vector = screen.at_pixel(x, y);
	let pixel_vector = pixel_vector.rotate_at_y(camera.yaw);
	let pixel_vector = pixel_vector.rotate_at_x(camera.pitch);
	let pixel_vector = pixel_vector.rotate_at_z(camera.roll);
	let pixel_ray = Ray::new(&camera.location, pixel_vector);
	//let color = raytracer::trace_ray(screen, lod, view_lod, pixel_ray, model, model.scale, max_distance);
	let color = raytracer::trace_ray_normals(screen, lod, view_lod, pixel_ray, model, model.scale, max_distance);
	color
}
*/

pub fn trace_pixel(lod:u8, view_lod:u8, model:&Model, screen:&Screen, camera:&Camera, x:i64, y:i64, max_distance:u64)->Color{
	let pixel_vector = screen.at_pixel(x, y);
	let pixel_vector = pixel_vector.rotate_at_y(camera.roll);
	let pixel_vector = pixel_vector.rotate_at_x(camera.pitch);
	let pixel_vector = pixel_vector.rotate_at_z(camera.yaw);
	let pixel_ray = Ray::new(&camera.eye, pixel_vector);
	//let color = raytracer::trace_ray(screen, lod, view_lod, pixel_ray, model, model.scale, max_distance);
	let color = raytracer::trace_ray_normals(screen, lod, view_lod, pixel_ray, model, model.scale, max_distance);
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

