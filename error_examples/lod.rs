extern crate balisong;

use std::num::Float;

use balisong::screen::Screen;

fn main(){
	println!("calculating LOD base on distance and fov");
	let screen = Screen::new(1920, 1080, 1920/2);
	//let screen = Screen::new(800, 600, 800/2);
	//let screen = Screen::new(1360, 768, 1360/2);
	println!("screen fov: {}", screen.fov.to_degrees());
	let view_lod = screen.get_view_lod();
	println!("view_lod: {}",view_lod);
	let distance = 1f64;
	let required_lod = screen.get_required_lod(view_lod, distance);
	println!("required_lod at distance {} is {}",distance, required_lod);
	
}
