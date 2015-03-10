use octree::Octree;
use point::Point;
use ray::Ray;
use model::Model;
use std::num::Float;
use color::Color;
use vector::Vector;

/*
pub fn trace(lod:u8, view_lod:u8, ray:Ray, model:&Model, max_distance:u64 )->Color{
	//println!("tracing a ray: {} with max distance: {} ", ray, max_distance);
	//println!("\t with a model: {}", model);
	
	let scale = (1 << (view_lod - lod)) as f64;
	
	let mut length = 0;
	while length < max_distance {
		let photon = ray.at_length(length);
		let octree = &model.octree;
		let model_loc = &model.location;
		
		let model_rel = Vector::new(model_loc.x as f64 / scale, model_loc.y as f64 / scale, model_loc.z as f64 / scale);
		let photon_relative = model_rel.subtract(photon);
		
		let px = (photon_relative.x / scale).round() as i64;
		let py = (photon_relative.y / scale).round() as i64;
		let pz = (photon_relative.z / scale).round() as i64;
		
		let hit = octree.is_point_occupied(lod, px, py, pz);
		if hit {
			//println!("hit at : {}",photon_relative);
			return octree.get_color(lod, px, py, pz);
		}
		length += 1;
	}
	Color::white()
}
*/

pub fn trace(lod:u8, view_lod:u8, ray:Ray, model:&Model, obj_scale:f64, max_distance:u64 )->Color{
	//let view_scale = (1 << (view_lod - lod)) as f64;//only applicable when view_lod > lod
	let limit = 1 << lod;
	let view_limit = 1 << view_lod;
	let view_scale = view_limit as f64 / limit as f64;
	
	let mut length = 0;
	while length < max_distance {
		let photon = ray.at_length(length);
		let model_loc = Vector::new(model.location.x as f64, model.location.y as f64, model.location.z as f64);
		let photon_rel = model_loc.subtract(photon);
		let photon_scale = photon_rel.scale(obj_scale/view_scale).round();
		
		let hit = model.octree.is_point_occupied(lod, photon_scale.x, photon_scale.y, photon_scale.z);
		if hit {
			return model.octree.get_color(lod, photon_scale.x, photon_scale.y, photon_scale.z);
		}
		length += 1;
	}
	Color::white()
}
