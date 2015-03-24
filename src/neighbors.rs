use vector::Vector;
use location;
use octree::Octree;
use point::Point;


/// get all the points that are neighbor to this location
pub  fn  get_neighbors<T> (node:&Octree<T>, lod:u8, point:&Point, neighbors_dir:&Vec<Vector>)->Vec<Point>{
	let vec_loc = Vector::from_point(point);
	let mut neighbors_loc = Vec::new();
	for i in 0..neighbors_dir.len(){
		let new_vec_loc = vec_loc.add(&neighbors_dir[i]);
		let vec_point = new_vec_loc.as_point();
		if node.is_point_occupied(lod, vec_point.x, vec_point.y, vec_point.z){
			neighbors_loc.push(vec_point);
		}
	}
	neighbors_loc
}

/// get all non-occluded neighbors
pub  fn  get_non_occluded_neighbors<T> (node:&Octree<T>, lod:u8, point:&Point, neighbors_dir:&Vec<Vector>)->Vec<Point>{
	let vec_loc = Vector::from_point(point);
	let mut neighbors_loc = Vec::new();
	for i in 0..neighbors_dir.len(){
		let new_vec_loc = vec_loc.add(&neighbors_dir[i]);
		let vec_point = new_vec_loc.as_point();
		if !is_occluded(node, lod, &vec_point) && node.is_point_occupied(lod, vec_point.x, vec_point.y, vec_point.z){
			neighbors_loc.push(vec_point);
		}
	}
	neighbors_loc
}

/// get all empty neighbors at this directions
pub  fn  get_empty_neighbors<T> (node:&Octree<T>, lod:u8, point:&Point, neighbors_dir:&Vec<Vector>)->Vec<Point>{
	let vec_loc = Vector::from_point(point);
	let mut neighbors_loc = Vec::new();
	for i in 0..neighbors_dir.len(){
		let new_vec_loc = vec_loc.add(&neighbors_dir[i]);
		let vec_point = new_vec_loc.as_point();
		if !node.is_point_occupied(lod, vec_point.x, vec_point.y, vec_point.z){
			neighbors_loc.push(vec_point);
		}
	}
	neighbors_loc
}


///return octree that is part of the 6 face neighbors, these are the closes neighbor
///            ================
///             6 face neighbors          
///            ================
///             0  0  1
///             0  1  0
///             1  0  0
///             0  0 -1
///             0 -1  0
///            -1  0  0
pub  fn  get_face_neighbors<T> (node:&Octree<T>, lod:u8, point:&Point)->Vec<Point>{
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

pub  fn  get_empty_face_neighbors<T> (node:&Octree<T>, lod:u8, point:&Point)->Vec<Point>{
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

/// return the array of octrees that fall on the 8 edges, these are the farthest neighbors
///            ================
///             8 edges         
///            ================        	
///            -1 -1 -1 
///            -1 -1  1
///            -1  1 -1
///            -1  1  1
///             1 -1 -1
///             1 -1  1
///             1  1 -1	
///             1  1  1 
pub fn get_edge_neighbors<T>(node:&Octree<T>, lod:u8, point:&Point)->Vec<Point>{
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

pub fn get_empty_edge_neighbors<T>(node:&Octree<T>, lod:u8, point:&Point)->Vec<Point>{
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

/// return the array of octree that falls on the 12 side neighbors, these are the second closes neighbors
///
///             ================
///             12 side neighbors       
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

pub fn get_side_neighbors<T>(node:&Octree<T>, lod:u8, point:&Point)->Vec<Point>{
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

pub fn get_empty_side_neighbors<T>(node:&Octree<T>, lod:u8, point:&Point)->Vec<Point>{
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


pub fn get_all_non_occluded_neighbors<T>(node:&Octree<T>, lod:u8, point:&Point)->Vec<Point>{
	let neighbor_loc = vec![
			//face neighbors
			Vector::new( 0.0, 0.0, 1.0),
			Vector::new( 0.0, 1.0, 0.0),
			Vector::new( 1.0, 0.0, 0.0),
			Vector::new( 0.0, 0.0,-1.0),
			Vector::new( 0.0,-1.0, 0.0),
			Vector::new(-1.0, 0.0, 0.0),
			//edge neighbors
			Vector::new(-1.0,-1.0,-1.0),
			Vector::new(-1.0,-1.0, 1.0),
			Vector::new(-1.0, 1.0,-1.0),
			Vector::new(-1.0, 1.0, 1.0),
			Vector::new( 1.0,-1.0,-1.0),
			Vector::new( 1.0,-1.0, 1.0),
			Vector::new( 1.0, 1.0,-1.0),
			Vector::new( 1.0, 1.0, 1.0),
			
			//side neighbors
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


pub fn get_all_neighbors<T>(node:&Octree<T>, lod:u8, point:&Point)->Vec<Point>{
	let neighbor_loc = vec![
			//face neighbors
			Vector::new( 0.0, 0.0, 1.0),
			Vector::new( 0.0, 1.0, 0.0),
			Vector::new( 1.0, 0.0, 0.0),
			Vector::new( 0.0, 0.0,-1.0),
			Vector::new( 0.0,-1.0, 0.0),
			Vector::new(-1.0, 0.0, 0.0),
	
			//edge neighbors
			Vector::new(-1.0,-1.0,-1.0),
			Vector::new(-1.0,-1.0, 1.0),
			Vector::new(-1.0, 1.0,-1.0),
			Vector::new(-1.0, 1.0, 1.0),
			Vector::new( 1.0,-1.0,-1.0),
			Vector::new( 1.0,-1.0, 1.0),
			Vector::new( 1.0, 1.0,-1.0),
			Vector::new( 1.0, 1.0, 1.0),
			
			//side neighbors
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

pub fn get_all_empty_neighbors<T>(node:&Octree<T>, lod:u8, point:&Point)->Vec<Point>{
	let neighbor_loc = vec![
			//face neighbors
			Vector::new( 0.0, 0.0, 1.0),
			Vector::new( 0.0, 1.0, 0.0),
			Vector::new( 1.0, 0.0, 0.0),
			Vector::new( 0.0, 0.0,-1.0),
			Vector::new( 0.0,-1.0, 0.0),
			Vector::new(-1.0, 0.0, 0.0),
	
			//edge neighbors
			Vector::new(-1.0,-1.0,-1.0),
			Vector::new(-1.0,-1.0, 1.0),
			Vector::new(-1.0, 1.0,-1.0),
			Vector::new(-1.0, 1.0, 1.0),
			Vector::new( 1.0,-1.0,-1.0),
			Vector::new( 1.0,-1.0, 1.0),
			Vector::new( 1.0, 1.0,-1.0),
			Vector::new( 1.0, 1.0, 1.0),
			
			//side neighbors
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
pub fn is_occluded<T>(node:&Octree<T>, lod:u8, point:&Point)->bool{
	//get_face_neighbors(node, lod, point).len() == 6 //completely occluded
	get_all_neighbors(node, lod, point).len() == 26 //26 neighbors
}

pub fn is_semi_occluded<T>(node:&Octree<T>, lod:u8, point:&Point)->bool{
	//get_face_neighbors(node, lod, point).len() == 6 ||
	//get_side_neighbors(node, lod, point).len() == 12 ||
	//get_edge_neighbors(node, lod, point).len() == 8
	get_all_neighbors(node, lod, point).len() > 12
}

//get the closes occluded neighbor
pub fn get_closest_occluded_neighbor(node:&Octree<bool>, lod:u8, point:&Point)->Option<Point>{
	let face_neighbors = get_face_neighbors(node, lod, point);
	let side_neighbors = get_side_neighbors(node, lod, point);
	let edge_neighbors = get_edge_neighbors(node, lod, point);
	
	for i in 0..face_neighbors.len(){
		if is_occluded(node, lod, &face_neighbors[i]){
			return Some((&face_neighbors[i]).clone());
		}
	}
	
	for j in 0..side_neighbors.len(){
		if is_occluded(node, lod, &side_neighbors[j]){
			return Some((&side_neighbors[j]).clone());
		}
	}
	
	for k in 0..edge_neighbors.len(){
		if is_occluded(node, lod, &edge_neighbors[k]){
			return Some((&edge_neighbors[k]).clone());
		}
	}
	None
}

pub fn get_closest_empty_neighbor(node:&Octree<bool>, lod:u8, point:&Point)->Option<Point>{
	let empty_face_neighbors = get_empty_face_neighbors(node, lod, point);
	let empty_side_neighbors = get_empty_side_neighbors(node, lod, point);
	let empty_edge_neighbors = get_empty_edge_neighbors(node, lod, point);
	
	for i in 0..empty_face_neighbors.len(){
		return Some((&empty_face_neighbors[i]).clone());
	}
	
	for j in 0..empty_side_neighbors.len(){
		return Some((&empty_side_neighbors[j]).clone());
	}
	
	for k in 0..empty_edge_neighbors.len(){
		return Some((&empty_edge_neighbors[k]).clone());
	}
	None
}