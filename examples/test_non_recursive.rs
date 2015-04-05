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
use balisong::camera::Camera;
use balisong::renderer;
use balisong::model::Model;
use balisong::octree::Octree;
use balisong::location;

fn main(){
	let mut node = Octree::new();
	let lod  = 5;
	let loc = location::from_xyz(lod, 1,2,3);
	location::display(&loc);
	node.set_tree_non_recursive(&loc, &mut Some(true));
}