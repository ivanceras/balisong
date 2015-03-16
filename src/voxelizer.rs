use shape::Shape;
use octree::Octree;
use location;
use std::num::Float;

//voxelize a shape into a required lod
pub fn voxelize<T:Shape> (required_lod:u8, shape:T)->Octree<bool>{
	let limit = 1 << required_lod;
	let mut root = Octree::new();
	let mut normals = Octree::new();

	
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
	let mut percentage = 0;
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
					let normal_loc = loc.clone();
					root.set_tree(loc, Some(true));//move voxel and location to the octree
					normals.set_tree(normal_loc, Some(shape.normal(x as i64, y as i64, z as i64)));
				}
			}
		}
	}
	root
}