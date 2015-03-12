//describe the scene of
use std::old_io::File;
use std::sync::mpsc;
use std::num::Float;
use std::thread::Thread;

use point::Point;
use camera::Camera;
use octree::Octree;
use screen::Screen;
use ray::Ray;
use raytracer;
use model::Model;
use color::Color;


pub struct Scene{
	objects:Vec<Model>,
	view_lod:u8,//lod resolution of the voxels
	camera:Camera,	
}

impl Scene{
	
	pub fn new(view_lod:u8)->Scene{
		Scene{
			objects:Vec::new(), 
			view_lod:view_lod,
			camera:Camera::default()
		}
	}
	
	pub fn set_camera(&mut self, cam:Camera){
		self.camera = cam;
	}
	
		//add an octree to the scene
	pub fn add(&mut self, x:i64, y:i64 ,z:i64, octree:Octree, scale:f64){
		let model = Model::new(Point::new(x,y,z), octree, scale);
		self.objects.push(model);
	}
	
	//takes a long time to render when in threading
	pub fn render_threaded(&self, lod:u8, screen:&Screen, model:Model, camera:&Camera)->Vec<Color>{
		raytracer::render_threaded(lod, self.view_lod, model, screen, camera)
	}
	
	//trace the ray from the camera then get the pixels
	pub fn render(&self, lod:u8, screen:&Screen)->Vec<Color>{
		let mut pixels = Vec::new();
		let view_limit = 1 << self.view_lod;
		let max_distance = 2 * (view_limit as f64 * view_limit as f64).sqrt().round() as u64;
		let mut percentage = 0;
		for y in 0..screen.height{
			let new_percentage = (y as f64 * 100.0 / screen.height as f64).round() as u64;
			if new_percentage > percentage{
				println!("{}%",new_percentage);
			}
			percentage = new_percentage;
			for x in 0..screen.width{
				let color = self.trace_pixel(lod, screen, x, y, max_distance);
				pixels.push(color);
			}	
		}
		pixels
	}
	
	pub fn trace_pixel(&self, lod:u8, screen:&Screen, x:i64, y:i64, max_distance:u64)->Color{
		let pixel_vector = screen.at_pixel(x, y);
		let pixelv_yaw_rotated = pixel_vector.rotate_at_y(self.camera.yaw);
		let pixelv_pitch_rotated = pixelv_yaw_rotated.rotate_at_x(self.camera.pitch);
		let pixelv_roll_rotated = pixelv_pitch_rotated.rotate_at_z(self.camera.roll);
		let pixel_ray = Ray::new(&self.camera.location, pixelv_roll_rotated);
		let model = &self.objects[0];
		let color = raytracer::trace_ray(lod, self.view_lod, pixel_ray, model, model.scale, max_distance);
		color
	}
	
	//save pixels to file
	pub fn save_to_file(&self, filename:String, pixels:Vec<Color>, width:i64, height:i64){
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
	
	pub fn render_to_file(&self, lod:u8, screen:&Screen, filename:String){
		let pixels = self.render(lod, screen);
		self.save_to_file(filename, pixels, screen.width, screen.height);
	}
	
}