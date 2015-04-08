//compute the location of x,y,z values based on the array of bytes, each byte describe the location of the bits
//calculate the location using the lod x,y,z 
use std::num::Float;
use vector::Vector;

use constants;
use lod::LOD;

pub fn from_xyz(lod:&LOD, x:u64, y:u64, z:u64)->Vec<u64>{
	let limit = lod.limit as u64;
	let mut index = x * limit * limit + y * limit + z;
	let mut location = Vec::new();
	for h in 1..lod.lod+1{
		location.push(0);
	}
	for i in (1..lod.lod+1).rev(){
		let rem = index % constants::BITS as u64;
		index = (index - rem) / constants::BITS as u64;
		let loc = 1 << rem;
		location[(i - 1) as usize] = loc;
	}
	location
}

/// compute the next loc at this point with the direction r
/// This is necessary to make the traversal of the ray much much faster
/// Need to use http://en.wikipedia.org/wiki/Bresenham's_line_algorithm
/// 3D version http://www.mathworks.com/matlabcentral/fileexchange/21057-3d-bresenhams-line-generation/content/bresenham_line3d.m

pub fn next(lod:&LOD, loc:&Vec<u64>, dir:&Vector)->Vec<u64>{
	Vec::new()
}

/*
pub fn from_xyz(lod:u8, x:u64, y:u64, z:u64)->Vec<u8>{
	let limit = 1 << lod;
	let mut index = x * limit * limit + y * limit + z;
	let mut location = vec![0;lod as usize];
	//for h in 1..lod+1{
	//	location.push(0);
	//}
	for i in (1..lod+1).rev(){
		let rem = index % 8;
		index = (index - rem) / 8 ;
		let loc = 1 << rem;
		let pos = (i - 1) as usize;
		location[pos] = loc;
	}
	location
}
*/

pub fn is_bounded(lod:&LOD, x:i64, y:i64, z:i64)->bool{
    	let limit = lod.limit as i64;
 		if x < 0 || y < 0 || z < 0 
 		|| x > limit || y > limit || z > limit
 		{
 			return false;
 		}
 		true
}


//from location notation convert to eulidean xyz coordinate
pub fn to_xyz(location:&Vec<u8>)->(u64, u64, u64){
	let mut index = 0u64;
	let lod = LOD::new(location.len() as u8);
	for i in 0..location.len(){
		let local_index = which_bit(location[i]);
		index = (constants::BITS as u64 * index )+local_index as u64;
	}
	index_to_xyz(&lod, index)
}

fn which_bit(byte:u8)->u8{
	(byte as f64).log(constants::BASE as f64) as u8
}

pub fn index_to_xyz(lod:&LOD, idx:u64)->(u64, u64, u64){
	let limit = lod.limit as u64;
	let mut index = idx;
    let z =  index % limit;
    index /= limit;
    let y = index % limit;
    index /= limit;
    let x = index;	
    (x, y, z)
}

pub fn index_to_zyx(lod:u8, idx:u64)->(u64, u64, u64){
	let limit = 1 << lod;
	let mut index = idx;
    let x =  index % limit;
    index /= limit;
    let y = index % limit;
    index /= limit;
    let z = index;	
    (x, y, z)
}

pub fn index_to_yzx(lod:u8, idx:u64)->(u64, u64, u64){
	let limit = 1 << lod;
	let mut index = idx;
    let y =  index % limit;
    index /= limit;
    let z = index % limit;
    index /= limit;
    let x = index;	
    (x, y, z)
}

pub fn index_to_xzy(lod:u8, idx:u64)->(u64, u64, u64){
	let limit = 1 << lod;
	let mut index = idx;
    let x =  index % limit;
    index /= limit;
    let z = index % limit;
    index /= limit;
    let y = index;	
    (x, y, z)
}

pub fn count_bits(arg:u8)->u8 {
    let mut count:u8 = 0;
    let mut x = arg;
    while x > 0 {
        x &= x-1;
        count += 1;
    }
    count
}

pub fn display(location:&Vec<u8>){
	for i in 0..location.len(){
		println!("location[{}]: {:8b}",i,location[i]);
	}
}

