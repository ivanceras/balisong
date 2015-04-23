extern crate balisong;

use balisong::voxel::voxstream::Voxstream;

fn main(){
	let stream:Voxstream<u64>  = Voxstream::new();
	let loc = vec![64, 1];
	stream.get_index(&loc);
}