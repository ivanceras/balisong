//compute the location of x,y,z values based on the array of bytes, each byte describe the location of the bits
//calculate the location using the lod x,y,z 

use constants;
use lod::LOD;
use morton;

///[FIXME] seems like rem is used for the as the bitshift
/// should it be index / bits
pub fn from_xyz_orig(lod:&LOD, x:u64, y:u64, z:u64)->Vec<u64>{
	let mut index = xyz_to_index(lod, x, y, z);
	let mut location = Vec::new();
	for _ in 1..lod.lod+1{
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

/// a twist to the original but making the encoding the bits to local morton
pub fn from_xyz(lod:&LOD, x:u64, y:u64, z:u64)->Vec<u64>{
    //let mut index = xyz_to_index(lod, x, y, z);
    let mut index = xyz_to_morton(lod, x, y,z);
    let mut location = Vec::new();
    for _ in 1..lod.lod+1{
        location.push(0);
    }
    for i in (1..lod.lod+1).rev(){
        let rem = index % constants::BITS as u64;
        index = (index - rem) / constants::BITS as u64;
        let loc = 1 << linear_to_morton_8(rem);
        //let loc = 1 << rem;
        location[(i - 1) as usize] = loc;
    }
    location
}

/// convert a linear bit to morton bit
fn linear_to_morton_8(linear:u64)->u8{
    let morton = [0,4,2,6,1,5,3,7];
    morton[linear as usize]
}

fn morton_to_linear_8(m:u64)->u8{
    let morton = vec![0,4,2,6,1,5,3,7];
    let mut index = 0;
    for i in morton{
        if i == m{
            return index as u8;
        }
        index += 1;
    }
    panic!("invalid morton code");
}


fn linear_to_morton_64(linear:u64)->u8{
    let morton = [0,4,32,36,2,6,34,38,16,20,48,52,18,22,50,54,1,5,33,37,3,7,35,39,17,21,49,53,19,23,51,55,8,12,40,44,10,14,42,46,24,28,56,60,26,30,58,62,9,13,41,45,11,15,43,47,25,29,57,61,27,31,59,63];
    morton[linear as usize]
}
// calculate the location base on x,y,z
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
pub fn to_xyz_orig(location:&Vec<u64>)->(u64, u64, u64){
	let mut index = 0u64;
	let lod = LOD::new(location.len() as u8);
	for i in 0..location.len(){
		let local_index = which_bit(location[i]);
		index = (constants::BITS as u64 * index )+local_index as u64;
	}
	index_to_xyz(&lod, index)
}

/// a twist to the original but making the encoding the bits to local morton
pub fn to_xyz(location:&Vec<u64>)->(u64, u64, u64){
    let mut index = 0u64;
    let lod = LOD::new(location.len() as u8);
    for i in 0..location.len(){
        let local_index = which_bit(location[i]);
        let linear_index = morton_to_linear_8(local_index as u64);
        index = (constants::BITS as u64 * index )+linear_index as u64;
    }
    index_to_xyz(&lod, index)
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


#[test]
fn test_fromxyz_limit(){
    let lod = LOD::new(5);
    let (x, y, z) = (lod.limit as u64 -1 , lod.limit as u64 -1, lod.limit as u64 -1);
    let loc = from_xyz(&lod, x, y, z);
    let loc2 = from_xyz2(&lod, x, y, z);
    assert_eq!(loc, loc2);
}
#[test]
fn test_fromxyz_test_morton_bits(){
    let lod = LOD::new(5);
    let (x, y, z) = (lod.limit as u64 -1 , lod.limit as u64 -2, lod.limit as u64 -1);
    let loc = from_xyz(&lod, x, y, z);
    let loc_morton = from_xyz_morton_bits(&lod, x, y, z);
    let (x1, y1, z1) = to_xyz_morton_bits(&loc_morton);
    //assert_eq!(loc, loc_morton);
    println!("({}, {}, {}) == ({}, {}, {})", x, y, z, x1, y1, z1);
    assert_eq!((x, y, z), (x1, y1, z1));
    panic!("values pls");

}