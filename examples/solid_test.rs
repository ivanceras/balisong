extern crate balisong;

use balisong::octree::Octree;
use balisong::location;

fn main(){
	println!("testing solids...");
	let mut node = Octree::new();
	let lod = 5;
	let limit = 1 << lod;
	for x in 0..limit{
		for y in 0..limit{
			for z in 0..limit{
				let loc =  location::from_xyz(lod, x, y, z);
				node.set_tree(loc, Some(true));//move voxel and location to the octree
			}
		}
	}
	
	let size = limit * limit * limit;
	let mut computed_nodes = size;
	for i in 1..lod{
		computed_nodes += size / (i as u64 * 8)
	}
	println!("computed nodes: {}", computed_nodes);
	println!("size: {}", size);
	println!("node count: {}",node.count_nodes());
	println!("leaf count: {}",node.count_leaf());
	println!("is_partial_solid: {}",node.is_partial_solid());
	println!("is_solid: {}",node.is_solid());
	println!("is_all_children_leaf : {}",node.is_all_children_leaf());
}