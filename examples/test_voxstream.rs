extern crate balisong;

use balisong::voxel::voxstream::Voxstream;

fn main(){
	let mut stream:Voxstream<u64>  = Voxstream::new();
	stream.insert_root();
	println!("total_ones: {}", stream.total_ones());
}