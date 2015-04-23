extern crate balisong;
use balisong::ray::Ray;
use balisong::vector::Vector;
use balisong::point::Point;
use balisong::lod::LOD;
use balisong::location;

fn main(){
	
	let origin = Point::new(1, 0, 0);
	let direction = Vector::new(1.0, 0.0, 0.0);
	let ray = Ray::new(&origin, &direction);
	let lod = LOD::new(5);
	let view_lod = LOD::new(8);
	
	let view_scale = (lod.limit as f64 / view_lod.limit as f64) as f64;
	println!("view_scale {}/{} : {}",lod.limit, view_lod.limit, view_scale);
	for i in 0..10{
		let lod_vec = ray.at_lod_length(&lod, &view_lod, i as f64);
		let vec = ray.at_length(i as f64);
		let scaled_vec = vec.scale(1.0/view_scale);
		println!("\nlength{}  lod:{} no_lod:{} scaled:{}",i, lod_vec, vec, scaled_vec);
		let lod_loc = location::from_xyz(&lod, lod_vec.x as u64, lod_vec.y as u64, lod_vec.z as u64);
		let loc = location::from_xyz(&lod, vec.x as u64, vec.y as u64, vec.z as u64);
		println!("with LOD");
		location::display(&lod_loc);
		println!("no LOD");
		location::display(&loc);
	}
}