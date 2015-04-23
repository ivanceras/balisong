use vector::Vector;
use location;
use voxel::voxtree::Voxtree;
use voxel::vox::Vox;
use voxel::voxbit::Voxbit;
use point::Point;
use lod::LOD;
use constants;


/// get all the points that are neighbor to this location
pub  fn  get_neighbors (node:&Voxbit, lod:&LOD, point:&Point, neighbors_dir:&Vec<Vector>)->Vec<Point>{
	let vec_loc = Vector::from_point(point);
	let mut neighbors_loc = Vec::new();
	for i in 0..neighbors_dir.len(){
		let neighbor_scaled = neighbors_dir[i].scale(constants::NEIGHBOR_RADIUS);
		let new_vec_loc = vec_loc.add(&neighbor_scaled);
		let vec_point = new_vec_loc.as_point();
		if location::is_bounded(lod, vec_point.x, vec_point.y, vec_point.z){
			let loc = location::from_xyz(lod, vec_point.x as u64, vec_point.y as u64, vec_point.z as u64);
			let (iteration, hit) = node.is_location_occupied(&loc);
			if hit{
				neighbors_loc.push(vec_point);
			}
		}
	}
	neighbors_loc
}

/// get all non-occluded neighbors
pub  fn  get_non_occluded_neighbors (node:&Voxbit, lod:&LOD, point:&Point, neighbors_dir:&Vec<Vector>)->Vec<Point>{
	let vec_loc = Vector::from_point(point);
	let mut neighbors_loc = Vec::new();
	for i in 0..neighbors_dir.len(){
		let neighbor_scaled = neighbors_dir[i].scale(constants::NEIGHBOR_RADIUS);
		let new_vec_loc = vec_loc.add(&neighbor_scaled);
		let vec_point = new_vec_loc.as_point();
		if location::is_bounded(lod, vec_point.x, vec_point.y, vec_point.z){
			let loc = location::from_xyz(lod, vec_point.x as u64, vec_point.y as u64, vec_point.z as u64);
			//let (iteration, hit) = node.is_location_occupied(&loc);
			//let (iteration, hit) = node.is_location_occupied_iterative(&loc);
			let (iteration, hit) = node.is_location_occupied(&loc);
			if !is_occluded(node, lod, &vec_point) && hit{
				neighbors_loc.push(vec_point);
			}
		}
	}
	neighbors_loc
}

/// get all empty neighbors at this directions
pub  fn  get_empty_neighbors (node:&Voxbit, lod:&LOD, point:&Point, neighbors_dir:&Vec<Vector>)->Vec<Point>{
	let vec_loc = Vector::from_point(point);
	let mut neighbors_loc = Vec::new();
	for i in 0..neighbors_dir.len(){
		let neighbor_scaled = neighbors_dir[i].scale(constants::NEIGHBOR_RADIUS);
		let new_vec_loc = vec_loc.add(&neighbor_scaled);
		let vec_point = new_vec_loc.as_point();
		if location::is_bounded(lod, vec_point.x, vec_point.y, vec_point.z){
			let loc = location::from_xyz(lod, vec_point.x as u64, vec_point.y as u64, vec_point.z as u64);
			let (iteration, hit) = node.is_location_occupied(&loc);
			if !hit{
				neighbors_loc.push(vec_point);
			}
		}
	}
	neighbors_loc
}


///return Voxtree that is part of the 6 face neighbors, these are the closes neighbor
///            ================
///             6 face neighbors          
///            ================
///             0  0  1
///             0  1  0
///             1  0  0
///             0  0 -1
///             0 -1  0
///            -1  0  0
pub  fn  get_face_neighbors (node:&Voxbit, lod:&LOD, point:&Point)->Vec<Point>{
	let neighbor_loc = vec![
			Vector::new( 0.0, 0.0, 1.0),
			Vector::new( 0.0, 1.0, 0.0),
			Vector::new( 1.0, 0.0, 0.0),
			Vector::new( 0.0, 0.0,-1.0),
			Vector::new( 0.0,-1.0, 0.0),
			Vector::new(-1.0, 0.0, 0.0)
	];
	
	get_neighbors(node, lod, point, &neighbor_loc)
}

pub  fn  get_empty_face_neighbors (node:&Voxbit, lod:&LOD, point:&Point)->Vec<Point>{
	let neighbor_loc = vec![
			Vector::new( 0.0, 0.0, 1.0),
			Vector::new( 0.0, 1.0, 0.0),
			Vector::new( 1.0, 0.0, 0.0),
			Vector::new( 0.0, 0.0,-1.0),
			Vector::new( 0.0,-1.0, 0.0),
			Vector::new(-1.0, 0.0, 0.0)
	];
	
	get_empty_neighbors(node, lod, point, &neighbor_loc)
}

/// return the array of Voxtrees that fall on the 8 vertexs, these are the farthest neighbors
///            ================
///             8 vertexs         
///            ================        	
///            -1 -1 -1 
///            -1 -1  1
///            -1  1 -1
///            -1  1  1
///             1 -1 -1
///             1 -1  1
///             1  1 -1	
///             1  1  1 
pub fn get_vertex_neighbors(node:&Voxbit, lod:&LOD, point:&Point)->Vec<Point>{
	let neighbor_loc = vec![
			Vector::new(-1.0,-1.0,-1.0),
			Vector::new(-1.0,-1.0, 1.0),
			Vector::new(-1.0, 1.0,-1.0),
			Vector::new(-1.0, 1.0, 1.0),
			Vector::new( 1.0,-1.0,-1.0),
			Vector::new( 1.0,-1.0, 1.0),
			Vector::new( 1.0, 1.0,-1.0),
			Vector::new( 1.0, 1.0, 1.0),
	];
	
	get_neighbors(node, lod, point, &neighbor_loc)
}

pub fn get_empty_vertex_neighbors (node:&Voxbit, lod:&LOD, point:&Point)->Vec<Point>{
	let neighbor_loc = vec![
			Vector::new(-1.0,-1.0,-1.0),
			Vector::new(-1.0,-1.0, 1.0),
			Vector::new(-1.0, 1.0,-1.0),
			Vector::new(-1.0, 1.0, 1.0),
			Vector::new( 1.0,-1.0,-1.0),
			Vector::new( 1.0,-1.0, 1.0),
			Vector::new( 1.0, 1.0,-1.0),
			Vector::new( 1.0, 1.0, 1.0),
	];
	
	get_empty_neighbors(node, lod, point, &neighbor_loc)
}

/// return the array of Voxtree that falls on the 12 edge neighbors, these are the second closes neighbors
///
///             ================
///             12 edge neighbors       
///             ================   
///             
///             0  1  1
///             1  0  1
///             1  1  0
///             
///             0 -1 -1
///            -1  0 -1
///            -1 -1  0
///
///            -1  1  0
///             0 -1  1
///             0  1 -1
///
///             1  0  1
///             1 -1  0
///             1  0 -1

pub fn get_edge_neighbors(node:&Voxbit, lod:&LOD, point:&Point)->Vec<Point>{
	let neighbor_loc = vec![
			Vector::new( 0.0, 1.0, 1.0),
			Vector::new( 1.0, 0.0, 1.0),
			Vector::new( 1.0, 1.0, 0.0),
			
			Vector::new( 0.0,-1.0,-1.0),
			Vector::new(-1.0, 0.0,-1.0),
			Vector::new(-1.0,-1.0, 0.0),
			
			Vector::new(-1.0, 1.0,-0.0),
			Vector::new( 0.0,-1.0, 1.0),
			Vector::new( 0.0, 1.0,-1.0),

			Vector::new( 1.0, 0.0, 1.0),
			Vector::new( 1.0,-1.0, 0.0),
			Vector::new( 1.0, 0.0,-1.0),

	];
	get_neighbors(node, lod, point, &neighbor_loc)	
}

pub fn get_empty_edge_neighbors(node:&Voxbit, lod:&LOD, point:&Point)->Vec<Point>{
	let neighbor_loc = vec![
			Vector::new( 0.0, 1.0, 1.0),
			Vector::new( 1.0, 0.0, 1.0),
			Vector::new( 1.0, 1.0, 0.0),
			
			Vector::new( 0.0,-1.0,-1.0),
			Vector::new(-1.0, 0.0,-1.0),
			Vector::new(-1.0,-1.0, 0.0),
			
			Vector::new(-1.0, 1.0,-0.0),
			Vector::new( 0.0,-1.0, 1.0),
			Vector::new( 0.0, 1.0,-1.0),

			Vector::new( 1.0, 0.0, 1.0),
			Vector::new( 1.0,-1.0, 0.0),
			Vector::new( 1.0, 0.0,-1.0),

	];
	get_empty_neighbors(node, lod, point, &neighbor_loc)	
}


pub fn get_all_non_occluded_neighbors(node:&Voxbit, lod:&LOD, point:&Point)->Vec<Point>{
	let neighbor_loc = vec![
			//face neighbors
			Vector::new( 0.0, 0.0, 1.0),
			Vector::new( 0.0, 1.0, 0.0),
			Vector::new( 1.0, 0.0, 0.0),
			Vector::new( 0.0, 0.0,-1.0),
			Vector::new( 0.0,-1.0, 0.0),
			Vector::new(-1.0, 0.0, 0.0),
			//vertex neighbors
			Vector::new(-1.0,-1.0,-1.0),
			Vector::new(-1.0,-1.0, 1.0),
			Vector::new(-1.0, 1.0,-1.0),
			Vector::new(-1.0, 1.0, 1.0),
			Vector::new( 1.0,-1.0,-1.0),
			Vector::new( 1.0,-1.0, 1.0),
			Vector::new( 1.0, 1.0,-1.0),
			Vector::new( 1.0, 1.0, 1.0),
			
			//edge neighbors
			Vector::new( 0.0, 1.0, 1.0),
			Vector::new( 1.0, 0.0, 1.0),
			Vector::new( 1.0, 1.0, 0.0),
			Vector::new( 0.0,-1.0,-1.0),
			Vector::new(-1.0, 0.0,-1.0),
			Vector::new(-1.0,-1.0, 0.0),
			Vector::new(-1.0, 1.0,-0.0),
			Vector::new( 0.0,-1.0, 1.0),
			Vector::new( 0.0, 1.0,-1.0),
			Vector::new( 1.0, 0.0, 1.0),
			Vector::new( 1.0,-1.0, 0.0),
			Vector::new( 1.0, 0.0,-1.0),

	];
	get_non_occluded_neighbors(node, lod, point, &neighbor_loc)	
}


pub fn get_all_neighbors(node:&Voxbit, lod:&LOD, point:&Point)->Vec<Point>{
	let neighbor_loc = vec![
			//face neighbors
			Vector::new( 0.0, 0.0, 1.0),
			Vector::new( 0.0, 1.0, 0.0),
			Vector::new( 1.0, 0.0, 0.0),
			Vector::new( 0.0, 0.0,-1.0),
			Vector::new( 0.0,-1.0, 0.0),
			Vector::new(-1.0, 0.0, 0.0),
	
			//vertex neighbors
			Vector::new(-1.0,-1.0,-1.0),
			Vector::new(-1.0,-1.0, 1.0),
			Vector::new(-1.0, 1.0,-1.0),
			Vector::new(-1.0, 1.0, 1.0),
			Vector::new( 1.0,-1.0,-1.0),
			Vector::new( 1.0,-1.0, 1.0),
			Vector::new( 1.0, 1.0,-1.0),
			Vector::new( 1.0, 1.0, 1.0),
			
			//edge neighbors
			Vector::new( 0.0, 1.0, 1.0),
			Vector::new( 1.0, 0.0, 1.0),
			Vector::new( 1.0, 1.0, 0.0),
			Vector::new( 0.0,-1.0,-1.0),
			Vector::new(-1.0, 0.0,-1.0),
			Vector::new(-1.0,-1.0, 0.0),
			Vector::new(-1.0, 1.0,-0.0),
			Vector::new( 0.0,-1.0, 1.0),
			Vector::new( 0.0, 1.0,-1.0),
			Vector::new( 1.0, 0.0, 1.0),
			Vector::new( 1.0,-1.0, 0.0),
			Vector::new( 1.0, 0.0,-1.0),

	];
	get_neighbors(node, lod, point, &neighbor_loc)	
}

pub fn get_all_empty_neighbors(node:&Voxbit, lod:&LOD, point:&Point)->Vec<Point>{
	let neighbor_loc = vec![
			//face neighbors
			Vector::new( 0.0, 0.0, 1.0),
			Vector::new( 0.0, 1.0, 0.0),
			Vector::new( 1.0, 0.0, 0.0),
			Vector::new( 0.0, 0.0,-1.0),
			Vector::new( 0.0,-1.0, 0.0),
			Vector::new(-1.0, 0.0, 0.0),
	
			//vertex neighbors
			Vector::new(-1.0,-1.0,-1.0),
			Vector::new(-1.0,-1.0, 1.0),
			Vector::new(-1.0, 1.0,-1.0),
			Vector::new(-1.0, 1.0, 1.0),
			Vector::new( 1.0,-1.0,-1.0),
			Vector::new( 1.0,-1.0, 1.0),
			Vector::new( 1.0, 1.0,-1.0),
			Vector::new( 1.0, 1.0, 1.0),
			
			//edge neighbors
			Vector::new( 0.0, 1.0, 1.0),
			Vector::new( 1.0, 0.0, 1.0),
			Vector::new( 1.0, 1.0, 0.0),
			Vector::new( 0.0,-1.0,-1.0),
			Vector::new(-1.0, 0.0,-1.0),
			Vector::new(-1.0,-1.0, 0.0),
			Vector::new(-1.0, 1.0,-0.0),
			Vector::new( 0.0,-1.0, 1.0),
			Vector::new( 0.0, 1.0,-1.0),
			Vector::new( 1.0, 0.0, 1.0),
			Vector::new( 1.0,-1.0, 0.0),
			Vector::new( 1.0, 0.0,-1.0),

	];
	get_empty_neighbors(node, lod, point, &neighbor_loc)	
}
/// returns true if all the 6 face neighbors are occupied
pub fn is_occluded(node:&Voxbit, lod:&LOD, point:&Point)->bool{
	//get_face_neighbors(node, lod, point).len() == 6 //completely occluded
	get_all_neighbors(node, lod, point).len() == 26 //26 neighbors
}

pub fn is_semi_occluded(node:&Voxbit, lod:&LOD, point:&Point)->bool{
	//get_face_neighbors(node, lod, point).len() == 6 ||
	//get_edge_neighbors(node, lod, point).len() == 12 ||
	//get_vertex_neighbors(node, lod, point).len() == 8
	get_all_neighbors(node, lod, point).len() > 12
}

//get the closes occluded neighbor
pub fn get_closest_occluded_neighbor(node:&Voxbit, lod:&LOD, point:&Point)->Option<Point>{
	let face_neighbors = get_face_neighbors(node, lod, point);
	let edge_neighbors = get_edge_neighbors(node, lod, point);
	let vertex_neighbors = get_vertex_neighbors(node, lod, point);
	
	for i in 0..face_neighbors.len(){
		if is_occluded(node, lod, &face_neighbors[i]){
			return Some((&face_neighbors[i]).clone());
		}
	}
	
	for j in 0..edge_neighbors.len(){
		if is_occluded(node, lod, &edge_neighbors[j]){
			return Some((&edge_neighbors[j]).clone());
		}
	}
	
	for k in 0..vertex_neighbors.len(){
		if is_occluded(node, lod, &vertex_neighbors[k]){
			return Some((&vertex_neighbors[k]).clone());
		}
	}
	None
}

/// the occluded point towards the center of the body the dot product of the two vectors is approaching 1.0
pub fn get_closest_occluded_neighbor_towards_center(node:&Voxbit, lod:&LOD, point:&Point)->Option<Point>{
	//println!("getting the closest occluded neighbor towards center...");
	let limit = lod.limit as i64;
	let center = Point::new(limit/2, limit/2, limit/2);
	let vec_center = Vector::from_point(&center).unit_vector();
	
	let face_neighbors = get_face_neighbors(node, lod, point);
	let edge_neighbors = get_edge_neighbors(node, lod, point);
	let vertex_neighbors = get_vertex_neighbors(node, lod, point);
	
	let max_dot = -1.0;
	let mut closest = None;
	for i in 0..face_neighbors.len(){
		//println!("i : {} of {}",i,face_neighbors.len());
		if is_occluded(node, lod, &face_neighbors[i]){
			let vec_point = Vector::from_point(&face_neighbors[i]).unit_vector();
			let dot = vec_center.dot(&vec_point);
			//println!("face dot: {} max_dot: {} dot > max_dot: {}",dot, max_dot, dot > max_dot);
			if dot > max_dot{
				//println!("Yes dot is greater than max_dot!");
				max_dot == dot;
				closest = Some((&face_neighbors[i]).clone());
			}
		}
	}
	
	for j in 0..edge_neighbors.len(){
		if is_occluded(node, lod, &edge_neighbors[j]){
			let vec_point = Vector::from_point(&edge_neighbors[j]).unit_vector();
			let dot = vec_center.dot(&vec_point);
			//println!("edge dot: {} max_dot: {} dot > max_dot: {}",dot, max_dot, dot > max_dot);
			if dot > max_dot{
				//println!("Yes dot is greater than max_dot!");
				max_dot == dot;
				closest = Some((&edge_neighbors[j]).clone());
			}
		}
	}
	
	for k in 0..vertex_neighbors.len(){
		if is_occluded(node, lod, &vertex_neighbors[k]){
			let vec_point = Vector::from_point(&vertex_neighbors[k]).unit_vector();
			let dot = vec_center.dot(&vec_point);
			//println!("vertex dot: {} max_dot: {} dot > max_dot: {}",dot, max_dot, dot > max_dot);
			if dot > max_dot{
				//println!("Yes dot is greater than max_dot!");
				max_dot == dot;
				closest = Some((&vertex_neighbors[k]).clone());
			}
		}
	}
	//let got = closest.clone();
	//if got.is_some(){
	//	println!("got {}", got.unwrap());
	//}
	closest
}

pub fn get_closest_empty_neighbor(node:&Voxbit, lod:&LOD, point:&Point)->Option<Point>{
	let empty_face_neighbors = get_empty_face_neighbors(node, lod, point);
	let empty_edge_neighbors = get_empty_edge_neighbors(node, lod, point);
	let empty_vertex_neighbors = get_empty_vertex_neighbors(node, lod, point);
	
	for i in 0..empty_face_neighbors.len(){
		return Some((&empty_face_neighbors[i]).clone());
	}
	
	for j in 0..empty_edge_neighbors.len(){
		return Some((&empty_edge_neighbors[j]).clone());
	}
	
	for k in 0..empty_vertex_neighbors.len(){
		return Some((&empty_vertex_neighbors[k]).clone());
	}
	None
}

/// closes point away from center's dot product approaches -1.0
pub fn get_closest_empty_neighbor_away_center(node:&Voxbit, lod:&LOD, point:&Point)->Option<Point>{
	let empty_face_neighbors = get_empty_face_neighbors(node, lod, point);
	let empty_edge_neighbors = get_empty_edge_neighbors(node, lod, point);
	let empty_vertex_neighbors = get_empty_vertex_neighbors(node, lod, point);
	
	let limit = lod.limit as i64;
	let center = Point::new(limit/2, limit/2, limit/2);
	let vec_center = Vector::from_point(&center).unit_vector();
	
	let min_dot = 1.0;
	let mut closest = None;
	
	for i in 0..empty_face_neighbors.len(){
		let vec_point = Vector::from_point(&empty_face_neighbors[i]).unit_vector();
		let dot = vec_center.dot(&vec_point);
		if dot < min_dot{
			closest = Some((&empty_face_neighbors[i]).clone())
		}
	}
	
	for j in 0..empty_edge_neighbors.len(){
		let vec_point = Vector::from_point(&empty_edge_neighbors[j]).unit_vector();
		let dot  = vec_center.dot(&vec_point);
		if dot < min_dot{
			closest = Some((&empty_edge_neighbors[j]).clone());
		}
	}
	
	for k in 0..empty_vertex_neighbors.len(){
		let vec_point = Vector::from_point(&empty_vertex_neighbors[k]).unit_vector();
		let dot = vec_center.dot(&vec_point);
		if dot < min_dot{
			closest = Some((&empty_vertex_neighbors[k]).clone());
		}
	}
	closest
}