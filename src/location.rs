//compute the location of x,y,z values based on the array of bytes, each byte describe the location of the bits
//calculate the location using the lod x,y,z 

use constants;
use lod::LOD;

pub fn from_xyz(lod:&LOD, x:u64, y:u64, z:u64)->Vec<u64>{
	let mut index = xyz_to_index(lod, x, y, z);
	//let mut index = xyz_to_morton(lod, x, y, z);//using morton
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


pub fn xyz_to_index(lod:&LOD, x:u64, y:u64, z:u64)->u64{
	let limit = lod.limit as u64;
	return  x * limit * limit + y * limit + z;
}

pub fn is_bounded(lod:&LOD, x:i64, y:i64, z:i64)->bool{
    	let limit = lod.limit as i64;
 		if x < 0 || y < 0 || z < 0 
 		|| x > limit || y > limit || z > limit
 		{
 			return false;
 		}
 		true
}

pub fn morton_to_xyz(lod:&LOD, morton:u64)->(u64, u64, u64){
	let mut x = 0;
	let mut y = 0;
	let mut z = 0;
	for i in 0..lod.lod {
		x |= ((morton & ( 1  << 3 * i + 0)) >> ((3 * i) + 0)-i);
		y |= ((morton & ( 1  << 3 * i + 1)) >> ((3 * i) + 1)-i);
		z |= ((morton & ( 1  << 3 * i + 2)) >> ((3 * i) + 2)-i);
	}
	(x, y, z)
}

pub fn xyz_to_morton(lod:&LOD, x:u64, y:u64, z:u64)->u64{
	let mut answer:u64 = 0;
	for i in 0..lod.lod {
		answer |= ((x & (1 << i)) << 2*i) | ((y & (1 << i)) << (2*i + 1)) | ((z & (1 << i)) << (2*i + 2));
	}
	answer
}


//from location notation convert to eulidean xyz coordinate
pub fn to_xyz(location:&Vec<u64>)->(u64, u64, u64){
	let mut index = 0u64;
	let lod = LOD::new(location.len() as u8);
	for i in 0..location.len(){
		let local_index = which_bit(location[i]);
		index = (constants::BITS as u64 * index )+local_index as u64;
	}
	index_to_xyz(&lod, index)
	//morton_to_xyz(&lod, index)
}

///TODO: this can be replaced with the lowestBit algorithm
//fn which_bit(byte:u64)->usize{
//	(byte as f64).log(constants::BASE as f64) as usize
//}

//fn which_bit(byte:u64)->u8{
//	(byte as f64).log2() as u8
//}

fn which_bit(bitset:u64)->usize{
	let bitset = bitset - 1;
	bitset.count_ones() as usize
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



