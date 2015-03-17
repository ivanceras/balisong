//simplify the voxel octree, traverse starting from the lowest detail of the tree
//remove fully opaque children
use octree::Octree;

pub fn optimize(root:&mut Octree, lod:u8){
	//count the number of holes, if has holes don't unset
	println!("optimizing voxels by counting the holes...");
	println!("root holes: {:8b}",root.value);
	let total_marked = mark_solids(root, lod, 1);
	println!("Marked solids: {}",total_marked);
	let marked_children = mark_all_children_solids(root);
	println!("\tMarked children: {}", marked_children);
	println!("Done optimizing...");
}

//do this at the highest LOD
fn mark_solids(node:&mut Octree, lod:u8, depth:u8)->u64{
	let mut marked = 0;
	if node.value == 255 && depth == lod{
		node.solid = true;
		node.children.clear();
		marked += 1;
	}
	for i in 0..node.children.len(){
		marked += mark_solids(&mut node.children[i], lod, depth+1);
	}
	marked
}

//do for each level starting from the highest LOD-1
fn mark_all_children_solids(node:&mut Octree)->u64{
	let mut marked = 0;
	if node.is_all_children_solid(){//&&lod == depth -1
		node.solid = true;
		node.children.clear();
		marked += 1;
	}
	for i in 0..node.children.len(){
		marked += mark_all_children_solids(&mut node.children[i]);
	}
	marked
}
