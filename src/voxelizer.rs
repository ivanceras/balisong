use shape::Shape;
use octree::Octree;
use location;
use color::Color;
use voxel::Voxel;
use std::num::Float;

//voxelize a shape into a required lod
pub fn voxelize<T:Shape> (required_lod:u8, shape:T)->Octree{
	let limit = 1 << required_lod;
	let mut root = Octree::new();
	let mut percentage = 0;
	
	/*
	let origin =  location::from_xyz(required_lod, 0, 0, 1);
	let normal = shape.normal(0, 0, 0);
	let vox = Voxel::new(Color::red(), normal, 0, 0);
	root.put_tree(origin, vox);
	
	let origin =  location::from_xyz(required_lod, 0, 0, 2);
	let normal = shape.normal(0, 0, 0);
	let vox = Voxel::new(Color::green(), normal, 0, 0);
	root.put_tree(origin, vox);
	
	let origin =  location::from_xyz(required_lod, 0, 0, 3);
	let normal = shape.normal(0, 0, 0);
	let vox = Voxel::new(Color::blue(), normal, 0, 0);
	root.put_tree(origin, vox);
	*/
	
	for x in 0..limit{
		let new_percentage = (x as f64 * 100.0 / limit as f64).round() as u64;
		if new_percentage > percentage{
			println!("{}%",new_percentage);
		}
		percentage = new_percentage;
		for y in 0..limit{
			for z in 0..limit{
				if shape.is_inside(x as i64, y as i64, z as i64){
					let loc =  location::from_xyz(required_lod, x, y, z);
					/*
					let r = 255 - ((x as f64 / limit as f64) * 255.0).round() as u8;
					let g = 255 - ((y as f64 / limit as f64) * 255.0).round() as u8;
					let b = 255 - ((z as f64 / limit as f64) * 255.0).round() as u8;
					*/
					let r = ((x as f64 / limit as f64) * 255.0).round() as u8;
					let g = ((y as f64 / limit as f64) * 255.0).round() as u8;
					let b = ((z as f64 / limit as f64) * 255.0).round() as u8;
					let color = Color{r:r,g:g,b:b,a:255};
					let normal = shape.normal(x as i64, y as i64, z as i64);
					let voxel = Voxel::new(color, normal, 0, 0);
					root.put_tree(loc, voxel);//move voxel and location to the octree
				}
			}
		}
	}
	root
}