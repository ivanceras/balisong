use std::sync::mpsc;
use std::thread::Thread;
use std::sync::Arc;
use time::PreciseTime;

use octree::Octree;
use point::Point;
use ray::Ray;
use model::Model;
use std::num::Float;
use color::Color;
use vector::Vector;
use screen::Screen;
use camera::Camera;
use location;

//TODO: use adaptive view_lod 
//that is: when the ray is getting longer, the view_lod will be lesser
pub fn trace_ray(screen:&Screen, lod:u8, view_lod:u8, ray:Ray, model:&Model, obj_scale:f64, max_distance:u64 )->Color{
	//let view_scale = (1 << (view_lod - lod)) as f64;//only applicable when view_lod > lod
	let limit = 1 << lod;
	let view_limit = 1 << view_lod;
	let view_scale = view_limit as f64 / limit as f64;
	
	let mut length = 0.0;
	while length < max_distance as f64{
		let photon = ray.at_length(length);
		let model_loc = Vector::new(model.location.x as f64, model.location.y as f64, model.location.z as f64);
		let photon_rel = model_loc.subtract(&photon);
		let photon_scale = photon_rel.scale(obj_scale/view_scale).as_point();
		if location::is_bounded(lod, photon_scale.x, photon_scale.y, photon_scale.z){ //no more bounds check if the camera is located inside the one-world octree
			let vec_location = location::from_xyz(lod, photon_scale.x as u64, photon_scale.y as u64, photon_scale.z as u64);
			let hit = model.octree.is_location_occupied(&vec_location);
		
			if hit {
				return Color::new(
					(255 - (255 * photon_scale.x/limit)) as u8, 
					(255 - (255 * photon_scale.y/limit)) as u8, 
					(255 - (255 * photon_scale.z/limit)) as u8);
			}
		}
		length += 1.0;
	}
	Color::white()
}


///
///
/// Using normal to calculate the intensity of light at such point
///
///
pub fn trace_ray_normals(screen:&Screen, lod:u8, view_lod:u8, ray:Ray, model:&Model, obj_scale:f64, max_distance:u64 )->Color{
	
	let use_normal = true;//use normal or color
	
	let limit = 1 << lod;
	let view_limit = 1 << view_lod;
	let view_scale = view_limit as f64 / limit as f64;
	
	//sun light
	//let light = Vector::new(view_limit as f64/4.0, view_limit as f64/2.0, view_limit as f64);
	let light = Vector::new(-view_limit as f64, -view_limit as f64, -view_limit as f64/4.0);
	
	let mut length = 0.0;
	while length < max_distance as f64{
		let photon = ray.at_length(length);
		let model_loc = Vector::new(model.location.x as f64, model.location.y as f64, model.location.z as f64);
		let photon_rel = model_loc.subtract(&photon);
		let photon_scale = photon_rel.scale(obj_scale/view_scale).as_point();
		if location::is_bounded(lod, photon_scale.x, photon_scale.y, photon_scale.z){ //no more bounds check if the camera is located inside the one-world octree
			let vec_location = location::from_xyz(lod, photon_scale.x as u64, photon_scale.y as u64, photon_scale.z as u64);
			let hit = model.octree.is_location_occupied(&vec_location);
		
			if hit {
				
				if use_normal{
					let normal = model.normal.get(&vec_location).clone().unwrap();
					let normal_vec = normal.unit_vector();
					let light_vec = light.subtract(&photon).unit_vector();
					let intensity = normal_vec.dot(&light_vec);
					let color = Color::new( (127.0 * (intensity + 1.0)).round() as u8, 
											(127.0 * (intensity + 1.0)).round() as u8, 
											(127.0 * (intensity + 1.0)).round() as u8);
	
					return color;
				}
				else{
					return Color::new(
						(255 - (255 * photon_scale.x/limit)) as u8, 
						(255 - (255 * photon_scale.y/limit)) as u8, 
						(255 - (255 * photon_scale.z/limit)) as u8);
				}
			}
		}
		length += 1.0;
	}
	Color::white()
}
