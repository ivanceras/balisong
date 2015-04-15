use std::sync::mpsc;
use std::thread::Thread;
use std::sync::Arc;
use time::PreciseTime;

use voxtree::Voxtree;
use point::Point;
use ray::Ray;
use model::Model;
use std::num::Float;
use color::Color;
use vector::Vector;
use screen::Screen;
use camera::Camera;
use location;
use lod::LOD;
use voxelizer;
use constants;


pub fn factored_trace_ray_normals(screen:&Screen, lod:&LOD, view_lod:&LOD, ray:Ray, model:&Model, obj_scale:f64, max_distance:u64 )->Color{
	let view_limit = view_lod.limit as i64;
	let light = Vector::new(-view_limit as f64 * 2.0, -view_limit as f64 * 2.0, view_limit as f64 * 2.0);
	let hit_loc = hit_location(screen, lod, view_lod, ray, model, obj_scale, max_distance);
	
	if hit_loc.is_some(){
		let hit_loc = hit_loc.unwrap();
		let normal = model.normal.get(&hit_loc).clone().unwrap();
		let normal_vec = normal.unit_vector();
		
		let (x,y,z) = location::to_xyz(&hit_loc);
		let photon = Vector::new(x as f64,y as f64,z as f64);
		
		let light_vec = light.subtract(&photon).unit_vector();
		let intensity = normal_vec.dot(&light_vec);
		
		let object_color = Color::new( (255.0/2.0 * (intensity + 1.0)).round() as u8, 
							(233.0/2.0 * (intensity + 1.0)).round() as u8, 
							(0.0/2.0 * (intensity + 1.0)).round() as u8);
		
		let color = Color::new( (127.0 * (intensity + 1.0)).round() as u8, 
										(127.0 * (intensity + 1.0)).round() as u8, 
										(127.0 * (intensity + 1.0)).round() as u8);
		
		let fcolor = blend(object_color, color);
		return fcolor;
	}
	else{
		return Color::white();
	}
	
}

pub fn hit_location(screen:&Screen, lod:&LOD, view_lod:&LOD, ray:Ray, model:&Model, obj_scale:f64, max_distance:u64)->Option<Vec<u64>>{
	
	let limit = lod.limit as i64;
	let view_limit = view_lod.limit as i64;
	let scale = obj_scale * limit as f64/view_limit as f64; //scale of object to LOD to view lod
	let mut length = 0.0;
	while length < max_distance as f64{
		let photon = ray.at_length(length);
		let model_loc = Vector::new(model.location.x as f64, model.location.y as f64, model.location.z as f64);
		let photon_rel = model_loc.subtract(&photon);
		let photon_scale = photon_rel.scale(scale).as_point();
		if location::is_bounded(lod, photon_scale.x, photon_scale.y, photon_scale.z){ //no more bounds check if the camera is located inside the one-world octree
			let vec_location = location::from_xyz(lod, photon_scale.x as u64, photon_scale.y as u64, photon_scale.z as u64);
			let (iteration, hit) = model.normal.is_location_occupied_iterative(&vec_location);
			if hit {
				return Some(vec_location);
			}
			else{
				//println!("no hit at iteration: {}", iteration);
			}
		}
		length += 1.0;
	}
	None
}

//blending the normal color with model color
fn blend(color1:Color, color2:Color)->Color{
	let red   =  ((color1.r as f32 + color2.r as f32 )/2.0).round() as u8;
	let green =  ((color1.g as f32 + color2.g as f32 )/2.0).round() as u8;
	let blue  =  ((color1.b as f32 + color2.b as f32 )/2.0).round() as u8;
	Color::new(red, green, blue)
}

//http://www.iquilezles.org/www/articles/outdoorslighting/outdoorslighting.htm
//http://stackoverflow.com/questions/16521003/gamma-correction-formula-gamma-or-1-gamma
//Corrected = 255 * (Image/255)^(1/2.2).
fn gamma_correction(color:Color)->Color{
	let gamma = 1.0/2.2;
	let red = (255.0 * (color.r as f64 / 255.0).powf(gamma)).round() as u8;
	let green = (255.0 * (color.g as f64 / 255.0).powf(gamma)).round() as u8;
	let blue = (255.0 * (color.b as f64 / 255.0).powf(gamma)).round() as u8;
	Color::new(red, green, blue)
}

