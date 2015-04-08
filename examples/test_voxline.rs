extern crate balisong;

use balisong::lod::LOD;
use balisong::voxline::Voxline;

fn main(){
	let lod = LOD::new(5);
	let vox = Voxline::new(lod);
}