use std::num::Float;

use neighbors;
use shape::Shape;
use octree::Octree;
use location;
use normal::Normal;
use point::Point;
use vector::Vector;

//voxelize a shape into a required lod
pub fn voxelize<T:Shape> (required_lod:u8, shape:T)->(Octree<bool>, Octree<Normal>){
	let limit = 1 << required_lod;
	let mut root = Octree::new();
	let recalculate_normals = true;
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
					root.set_tree(&loc, Some(true));//move voxel and location to the octree
						if !recalculate_normals{
							normals.set_tree(&loc, Some(shape.normal(x as i64, y as i64, z as i64)));
						}
				}
			}
		}
	}
	if recalculate_normals{
		normals = calculate_normals(&root, required_lod);
	}
	(root, normals)
}

/// for all the points that is not complete occluded
/// calculate the normal

pub fn calculate_normals(node:&Octree<bool>, lod:u8)->Octree<Normal>{
	let limit = 1 << lod;
	let mut normals = Octree::new();
	let mut percentage = 0;
	println!("Calculating normals...");
	for x in 0..limit{
		let new_percentage = (x as f64 * 100.0 / limit as f64).round() as u64;
		if new_percentage > percentage{
			println!("{}%",new_percentage);
		}
		percentage = new_percentage;
		for y in 0..limit{
			for z in 0..limit{
				let point = Point::new(x as i64, y as i64, z as i64);
				let loc =  location::from_xyz(lod, x, y, z);
				//if node.is_location_occupied(&loc) && !neighbors::is_occluded(node, lod, &point){
				if node.is_location_occupied(&loc){
					let normal = calculate_point_normal(node, lod, &point);
					normals.set_tree(&loc, Some(normal));
				}
			}
		}
	}
	normals
}

/// get the closest occluded point
/// get all the neigbors,
/// get the cross product of 2 vectors neighbors at a time

fn calculate_point_normal(node:&Octree<bool>, lod:u8, point:&Point)->Normal{
	if neighbors::is_occluded(node, lod, point){
		//println!("This is an occluded point!, This shouldnt get a normal");
		return Normal::from_vector(&Vector::new(0.0, 0.0, 0.0));
	}
	let occluded = neighbors::get_closest_occluded_neighbor(node, lod, point);
	let closest_empty = neighbors::get_closest_empty_neighbor(node, lod, point);
	
	
	let occlusion_normal = if occluded.is_some(){
		let normal = calculate_normal_based_occluded_point(node, lod, point, &(occluded.unwrap()));
		Some(normal)
	}else{
		None
	};
	
	let empty_neighbor_normal = if closest_empty.is_some(){
		let normal = calculate_normal_based_empty_neighbor(node, lod, point, &(closest_empty.unwrap()));
		Some(normal)
	}else{
 		None
 	};

	//if occlusion_normal.is_some() &&  empty_neighbor_normal.is_some(){
	//	let average_normal = get_average(&vec![occlusion_normal.unwrap(), empty_neighbor_normal.unwrap()]);
	//	return Normal::from_vector(&average_normal);
	//}
	//else{
		if occlusion_normal.is_some(){
			return Normal::from_vector(&occlusion_normal.unwrap());
		}
		if empty_neighbor_normal.is_some(){
			return Normal::from_vector(&empty_neighbor_normal.unwrap());
		}
	//}
	return Normal::from_vector(&Vector::new(0.0, 0.0, 0.0));
}


fn calculate_normal_based_occluded_point(node:&Octree<bool>, lod:u8, point:&Point, occluded:&Point)->Vector{
	let neighbors = neighbors::get_all_non_occluded_neighbors(node, lod, point);
	let vec_occluded = Vector::from_point(point).subtract_point(occluded).unit_vector();
	
	let mut normals = Vec::new();
	let mut skipped = 0;
	for i in 0..neighbors.len(){
		let pair0 = &neighbors[i];
		for j in 0..neighbors.len(){
			if i != j {
				let pair1 = &neighbors[j];
				let vec0 = Vector::from_point(&pair0).subtract_point(point).unit_vector();
				let vec1 = Vector::from_point(&pair1).subtract_point(point).unit_vector();
				//println!("pair: {} {} vector: {} {}", pair0, pair1, vec0, vec1);
		
				let normal = vec0.cross(&vec1).unit_vector();
				let distance = normal.distance();
				if distance > 0.0 {
					//println!("distance: {}", distance);
					let dot = vec_occluded.dot(&normal);
					if dot > 0.0{
						//println!("wrong direction, negate the normal");
						normals.push(normal.negate().unit_vector());
					}
					else{
						normals.push(normal.unit_vector());
					}
				}
				else{
					//println!("not enough distance for the cross product!.. skipping");
					skipped += 1;
				}
			}
		}
	}
	//println!("normals sample: {} skipped normals: {}",normals.len(), skipped);
	let vec_normal = get_average(&normals).unit_vector();
	vec_normal
}

fn calculate_normal_based_empty_neighbor(node:&Octree<bool>, lod:u8, point:&Point, closest_empty:&Point)->Vector{
	//println!("Calculating normal base on empty neighbor...");
	let empty_neighbors = neighbors::get_all_empty_neighbors(node, lod, point);
	let vec_closest = Vector::from_point(point).subtract_point(closest_empty).unit_vector();
	
	let mut normals = Vec::new();
	let mut skipped = 0;
	for i in 0..empty_neighbors.len(){
		let pair0 = &empty_neighbors[i];
		for j in 0..empty_neighbors.len(){
			if i != j {
				let pair1 = &empty_neighbors[j];
				let vec0 = Vector::from_point(&pair0).subtract_point(point).unit_vector();
				let vec1 = Vector::from_point(&pair1).subtract_point(point).unit_vector();
				let normal = vec0.cross(&vec1).unit_vector();
				let distance = normal.distance();
				if distance > 0.0 {
					let dot = vec_closest.dot(&normal);
					if dot < 0.0 { //is pointing on the solid side, negate
						normals.push(normal.negate().unit_vector());
					}
					else{
						normals.push(normal.unit_vector());
					}
				}
				else{
					//println!("not enough distance for the cross product!.. skipping");
					skipped += 1;
				}
			}
		}
	}
	let vec_normal = get_average(&normals).unit_vector();
	vec_normal

}

fn get_average(vectors:&Vec<Vector>)->Vector{
	let len = vectors.len();
	//assert!(len > 0, "vector length must be greater than 0");
	let mut xt = 0.0f64;
	let mut yt = 0.0f64;
	let mut zt = 0.0f64;
	for i in 0..len{
		xt += vectors[i].x;
		yt += vectors[i].y;
		zt += vectors[i].z;
	}
	//println!("totals: {}, {}, {}", xt, yt, zt);
	let ave = Vector::new(xt/len as f64, yt/len as f64, zt/len as f64);
	//println!("average: {}",ave);
	ave
}

///recalculate the normals by averaging the normals at each neighbor
fn smoothen_normals(node:&Octree<bool>, lod:u8)->Octree<Normal>{
	let limit = 1 << lod;
	let mut normals = Octree::new();
	let mut percentage = 0;
	println!("Calculating normals...");
	for x in 0..limit{
		let new_percentage = (x as f64 * 100.0 / limit as f64).round() as u64;
		if new_percentage > percentage{
			println!("{}%",new_percentage);
		}
		percentage = new_percentage;
		for y in 0..limit{
			for z in 0..limit{
				let point = Point::new(x as i64, y as i64, z as i64);
				let loc =  location::from_xyz(lod, x, y, z);
				if node.is_location_occupied(&loc){
					let normal = get_average_normal(node, lod, &point);
					normals.set_tree(&loc, Some(normal));
				}
			}
		}
	}
	normals
}

fn get_average_normal(node:&Octree<bool>, lod:u8, point:&Point)->Normal{
	Normal::from_vector(&Vector::new(1.0, 0.0, 0.0))
}
