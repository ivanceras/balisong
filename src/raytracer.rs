use std::sync::mpsc;
use std::thread::Thread;
use std::sync::Arc;

use octree::Octree;
use point::Point;
use ray::Ray;
use model::Model;
use std::num::Float;
use color::Color;
use vector::Vector;
use screen::Screen;
use camera::Camera;

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
		let photon_scale = photon_rel.scale(obj_scale/view_scale).round();
		
		let hit = model.octree.is_point_occupied(lod, photon_scale.x, photon_scale.y, photon_scale.z);
		if hit {
			return Color::new(
				(255 - (255 * photon_scale.x/limit)) as u8, 
				(255 - (255 * photon_scale.y/limit)) as u8, 
				(255 - (255 * photon_scale.z/limit)) as u8);
		}
		length += 1.0;
	}
	Color::white()
}


//TODO: use adaptive view_lod 
//that is: when the ray is getting longer, the view_lod will be lesser
pub fn trace_ray2(screen:&Screen, lod:u8, view_lod:u8, ray:Ray, model:&Model, obj_scale:f64, max_distance:u64 )->Color{
	//let view_scale = (1 << (view_lod - lod)) as f64;//only applicable when view_lod > lod
	let mut length = 0.0f64;
	let mut required_lod = screen.get_required_lod(view_lod, length as f64) as u8;
	let mut limit = 1 << required_lod;
	let mut view_limit = 1 << view_lod;
	let mut view_scale = view_limit as f64 / limit as f64;
	let mut max_distance = 2.0 * (view_limit as f64 * view_limit as f64).sqrt().round();
	let mut max_distance = 8.0 * (limit as f64 * limit as f64).sqrt().round();


	while length < max_distance {
		let photon = ray.at_length(length);
		let model_loc = Vector::new(model.location.x as f64, model.location.y as f64, model.location.z as f64);
		let photon_rel = model_loc.subtract(&photon);
		let photon_scale = photon_rel.scale(obj_scale/view_scale).round();
		
		let hit = model.octree.is_point_occupied(required_lod, photon_scale.x, photon_scale.y, photon_scale.z);
		if hit {
			/*
			return Color::new(
				(255 - (255 * photon_scale.x/limit)) as u8, 
				(255 - (255 * photon_scale.y/limit)) as u8, 
				(255 - (255 * photon_scale.z/limit)) as u8);
			*/
			return Color::red();
		}
		let new_required_lod = screen.get_required_lod(view_lod, length as f64) as u8;
		if new_required_lod != required_lod {
			//println!("new required lod: {}", new_required_lod);
				limit = 1 << required_lod;
				view_limit = 1 << view_lod;
				view_scale = view_limit as f64 / limit as f64;
				//max_distance = 2.0 * (view_limit as f64 * view_limit as f64).sqrt().round();
				max_distance = 8.0 * (limit as f64 * limit as f64).sqrt().round();
		}
		required_lod = new_required_lod;
		length += view_scale;
	}
	Color::white()
}