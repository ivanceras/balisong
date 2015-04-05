use std::num::Float;

use neighbors;
use shape::Shape;
use voxtree::Voxtree;
use location;
use normal::Normal;
use point::Point;
use vector::Vector;
use lod::LOD;

//voxelize a shape into a required lod
pub fn voxelize<T:Shape> (required_lod:&LOD, shape:T)->(Voxtree<bool>, Voxtree<Normal>){
	
	let limit = required_lod.limit as u64;
	let mut root = Voxtree::new();
	let mut normals = Voxtree::new();

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
					//root.set_tree(&loc, &mut Some(true));//move voxel and location to the Voxtree
					root.set_tree_non_recursive(&loc, &mut Some(true));//move voxel and location to the Voxtree
				}
			}
		}
	}
	normals = calculate_normals(&root, required_lod);
	(root, normals)
}

/// for all the points that is not complete occluded
/// calculate the normal

pub fn calculate_normals(node:&Voxtree<bool>, lod:&LOD)->Voxtree<Normal>{
	let use_smooth_normals = true;
	let smoothing_iteration = 1;//2 is enough
	
	let limit = lod.limit as u64;
	let mut normals = Voxtree::new();
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
				if node.is_location_occupied(&loc) && !neighbors::is_occluded(node, lod, &point){
				//if node.is_location_occupied(&loc){
					let normal = calculate_point_normal(node, lod, &point);
					//normals.set_tree(&loc, &mut Some(normal));
					normals.set_tree_non_recursive(&loc, &mut Some(normal));
				}
			}
		}
	}
	if use_smooth_normals{
		for i in 0..smoothing_iteration{
			println!("Pass {}.. ",i);
			normals = smoothen_normals(&node, &normals, lod);
		}
	}
	normals
}

/// get the closest occluded point
/// get all the neigbors,
/// get the cross product of 2 vectors neighbors at a time

pub fn calculate_point_normal(node:&Voxtree<bool>, lod:&LOD, point:&Point)->Normal{
	//let occluded = neighbors::get_closest_occluded_neighbor(node, lod, point);
	//using center bias
	let occluded = neighbors::get_closest_occluded_neighbor_towards_center(node, lod, point);
	if occluded.is_some(){
		let normal = calculate_normal_based_occluded_point(node, lod, point, &(occluded.unwrap()));
		return Normal::from_vector(&normal)
	}
	//let closest_empty = neighbors::get_closest_empty_neighbor(node, lod, point);
	//using center bias
	let closest_empty = neighbors::get_closest_empty_neighbor_away_center(node, lod, point);
	if closest_empty.is_some(){
		let normal = calculate_normal_based_empty_neighbor(node, lod, point, &(closest_empty.unwrap()));
		return Normal::from_vector(&normal)
	}
	println!("no occluded no empty....");
	let vec_normal = get_normal_from_center(lod, point);
	return Normal::from_vector(&vec_normal);
}

fn get_normal_from_center(lod:&LOD, point:&Point)->Vector{
	let limit = lod.limit as i64;
	let center = Point::new(limit/2, limit/2, limit/2);
	let vec_center = Vector::from_point(&center);
	let vec_point = Vector::from_point(&point);
	let normal = vec_point.subtract(&vec_center).negate();
	normal.unit_vector()
}


fn calculate_normal_based_occluded_point(node:&Voxtree<bool>, lod:&LOD, point:&Point, occluded:&Point)->Vector{
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

fn calculate_normal_based_empty_neighbor(node:&Voxtree<bool>, lod:&LOD, point:&Point, closest_empty:&Point)->Vector{
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
	if vec_normal.distance() > 0.0 {
		return 	vec_normal;
	}
	else{
		let vec_normal = get_normal_from_center(lod, point);
		return vec_normal;
	}

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
pub fn smoothen_normals(node:&Voxtree<bool>, initial_normals:&Voxtree<Normal>, lod:&LOD)->Voxtree<Normal>{
	let limit = lod.limit as u64;
	let mut normals = Voxtree::new();
	let mut percentage = 0;
	println!("Smoothing normals...");
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
				if node.is_location_occupied(&loc) && !neighbors::is_occluded(node, lod, &point){
				//if node.is_location_occupied(&loc){
					let normal = get_average_normal(node, initial_normals, lod, &point);
					//normals.set_tree(&loc, &mut Some(normal));
					normals.set_tree_non_recursive(&loc, &mut Some(normal));
				}
			}
		}
	}
	normals
}

fn get_average_normal(node:&Voxtree<bool>, normals:&Voxtree<Normal>, lod:&LOD, point:&Point)->Normal{
	let neighbors  = neighbors::get_all_non_occluded_neighbors(node, lod, point);
	let mut i_normals = Vec::new();
	for i in 0..neighbors.len(){
		let loc = location::from_xyz(lod, neighbors[i].x as u64, neighbors[i].y as u64, neighbors[i].z as u64);
		let i_normal = normals.get(&loc);
		if i_normal.is_some(){
			let i_normal = i_normal.clone();
			i_normals.push(i_normal.unwrap().unit_vector());
		}
	}
	let ave_normal = get_average(&i_normals);
	Normal::from_vector(&ave_normal)
}
