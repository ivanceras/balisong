use neighbors;
use shape::Shape;
use voxel::voxtree::Voxtree;
use voxel::voxbit::Voxbit;
use voxel::vox::Vox;
use location;
use normal::Normal;
use point::Point;
use vector::Vector;
use lod::LOD;

/// voxelize a shape into a required lod
pub fn voxelize<T:Shape> (required_lod:&LOD, shape:T)->(Voxtree<Normal>){
	
	let limit = required_lod.limit as u64;
	let mut root = Voxbit::new();

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
					//root.set_content(&loc, &mut Some(true));//move voxel and location to the Voxtree
					root.set_location(&loc);//move voxel and location to the Voxtree
				}
			}
		}
	}
	let normals = calculate_normals(&root, required_lod);
	normals
}

/// for all the points that is not complete occluded
/// calculate the normal

pub fn calculate_normals(node:&Voxbit, lod:&LOD)->Voxtree<Normal>{
	let mut cnt = 0;
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
				let (iteration, hit) = node.is_location_occupied(&loc);
				if  hit && !neighbors::is_occluded(node, lod, &point){
					let normal = calculate_point_normal(node, lod, &point);
					//normals.set_tree(&loc, &mut Some(normal));
					normals.set_content(&loc, &mut Some(normal));
					cnt += 1;
				}
			}
		}
	}
	println!("There are {} total voxels after carving..",cnt);
	normals
}


pub fn calculate_average_normals(node:&mut Voxtree<Normal>){
	let mut stack = Vec::new();
	stack.push(node);
	let mut cnt = 0;
	while stack.len() > 0{
		//let top = match stack.pop(){
		//	Some(x) => x,
		//	None => panic!("Error here"),
		//};
		let top = stack.remove(0);
		let mut normals = Vec::new();
		let mut has_normals = 0;
		let children_len = top.children.len();
		for child in &mut top.children{
			if child.content.is_some(){
				cnt += 1;
				has_normals += 1;
				let vec_normal = child.content.clone().unwrap().unit_vector();
				normals.push(vec_normal);
			}
			stack.push(child);
		}

		if has_normals > 0 && has_normals == children_len {
			let ave_normal  = get_average(&normals);
			println!("average normals..{}  {}",ave_normal, children_len);
			top.content = Some(Normal::from_vector(&ave_normal));
		}
	}
	println!("There are {} nodes has normals..",cnt);
}

/// get the closest occluded point
/// get all the neigbors,
/// get the cross product of 2 vectors neighbors at a time

pub fn calculate_point_normal(node:&Voxbit, lod:&LOD, point:&Point)->Normal{
	let occluded = neighbors::get_closest_occluded_neighbor_towards_center(node, lod, point);
	if occluded.is_some(){
		let normal = calculate_normal_based_occluded_point(node, lod, point, &(occluded.unwrap()));
		return Normal::from_vector(&normal)
	}
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


fn calculate_normal_based_occluded_point(node:&Voxbit, lod:&LOD, point:&Point, occluded:&Point)->Vector{
	let neighbors = neighbors::get_all_non_occluded_neighbors(node, lod, point);
	let vec_occluded = Vector::from_point(point).subtract_point(occluded);//no normalization
	
	let mut normals = Vec::new();
	let mut skipped = 0;
	for i in 0..neighbors.len(){
		let pair0 = &neighbors[i];
		for j in 0..neighbors.len(){
			if i != j {
				let pair1 = &neighbors[j];
				let vec0 = Vector::from_point(&pair0).subtract_point(point);
				let vec1 = Vector::from_point(&pair1).subtract_point(point);
		
				let normal = vec0.cross(&vec1);
				let distance = normal.distance();
				if distance > 0.0 {
					let dot = vec_occluded.dot(&normal);
					if dot > 0.0{
						normals.push(normal.negate());
					}
					else{
						normals.push(normal);
					}
				}
				else{
					skipped += 1;
				}
			}
		}
	}
	let vec_normal = get_average(&normals).unit_vector();
	vec_normal
}

fn calculate_normal_based_empty_neighbor(node:&Voxbit, lod:&LOD, point:&Point, closest_empty:&Point)->Vector{
	let empty_neighbors = neighbors::get_all_empty_neighbors(node, lod, point);
	let vec_closest = Vector::from_point(point).subtract_point(closest_empty);
	
	let mut normals = Vec::new();
	let mut skipped = 0;
	for i in 0..empty_neighbors.len(){
		let pair0 = &empty_neighbors[i];
		for j in 0..empty_neighbors.len(){
			if i != j {
				let pair1 = &empty_neighbors[j];
				let vec0 = Vector::from_point(&pair0).subtract_point(point);
				let vec1 = Vector::from_point(&pair1).subtract_point(point);
				let normal = vec0.cross(&vec1);
				let distance = normal.distance();
				if distance > 0.0 {
					let dot = vec_closest.dot(&normal);
					if dot < 0.0 { //is pointing on the solid side, negate
						normals.push(normal.negate());
					}
					else{
						normals.push(normal);
					}
				}
				else{
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
	let mut xt = 0.0f64;
	let mut yt = 0.0f64;
	let mut zt = 0.0f64;
	for i in 0..len{
		xt += vectors[i].x;
		yt += vectors[i].y;
		zt += vectors[i].z;
	}
	let ave = Vector::new(xt/len as f64, yt/len as f64, zt/len as f64);
	ave
}

///recalculate the normals by averaging the normals at each neighbor
pub fn smoothen_normals(node:&Voxbit, initial_normals:&Voxtree<Normal>, lod:&LOD)->Voxtree<Normal>{
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
				//let (iteration, hit) = initial_normals.is_location_occupied(&loc);
				//let (iteration, hit) = initial_normals.is_location_occupied_iterative(&loc);
				let (iteration, hit) = initial_normals.is_location_occupied(&loc);
				if hit && !neighbors::is_occluded(node,  lod, &point){
					let normal = get_average_normal(node, initial_normals, lod, &point);
					//normals.set_tree_iterative(&loc, &mut Some(normal));
					normals.set_content(&loc, &mut Some(normal));
				}
			}
		}
	}
	normals
}

///
///Create a mipmap normals for the lower level LOD's based on the highest LOD
/// This is done by traversing from the highest LOD then setting the average normals
///
///
pub fn mipmap_voxel_normals(normals:&Voxtree<Normal>)->Voxtree<Normal>{
	let mut normals = Voxtree::new();
	normals
}

fn get_average_normal(node:&Voxbit, normals:&Voxtree<Normal>, lod:&LOD, point:&Point)->Normal{
	let neighbors  = neighbors::get_all_non_occluded_neighbors(node, lod, point);
	let mut i_normals = Vec::new();
	for i in 0..neighbors.len(){
		let loc = location::from_xyz(lod, neighbors[i].x as u64, neighbors[i].y as u64, neighbors[i].z as u64);
		//let i_normal = normals.get(&loc);
		let i_normal = normals.get_content(&loc);
		if i_normal.is_some(){
			let i_normal = i_normal.clone();
			i_normals.push(i_normal.unwrap().unit_vector());
		}
	}
	let ave_normal = get_average(&i_normals);
	Normal::from_vector(&ave_normal)
}

///remove occluded points from the voxtree, this is to optimize memory consumption, do this after normals has been calculated
pub fn carve_out(node:&Voxbit, lod:&LOD)->Voxtree<bool>{
	let mut carved =Voxtree::new();
	let limit = lod.limit as u64;
	let mut percentage = 0;
	println!("Carving out...");
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
				//let (iteration, hit) = node.is_location_occupied(&loc);
				//let (iteration, hit) = node.is_location_occupied_iterative(&loc);
				let (iteration, hit) = node.is_location_occupied(&loc);
				if hit && !neighbors::is_occluded(node, lod, &point){
					//carved.set_tree_iterative(&loc, &mut Some(true));
					carved.set_content(&loc, &mut Some(true));
				}
			}
		}
	}
	carved
}
