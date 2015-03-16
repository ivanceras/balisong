
//compute the location of x,y,z values based on the array of bytes, each byte describe the location of the bits
//calculate the location using the lod x,y,z 
pub fn from_xyz(lod:u8, x:u64, y:u64, z:u64)->Vec<u8>{
	let limit = 1 << lod;
	let mut index = x * limit * limit + y * limit + z;
	let mut location = Vec::new();
	for h in 1..lod+1{
		location.push(0);
	}
	for i in (1..lod+1).rev(){
		let rem = index % 8;
		index = (index - rem) / 8 ;
		let loc = 1 << rem;
		location[(i - 1) as usize] = loc;
	}
	location
}

pub fn is_bounded(lod:u8, x:i64, y:i64, z:i64)->bool{
    	let limit = 1 << lod;
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
	let lod = location.len() as u8;
	for i in 0..location.len(){
		let local_index = which_bit(location[i]);
		index = (8*index)+local_index as u64;
	}
	index_to_xyz(lod, index)
}

//same as byte.log2()
fn which_bit(byte:u8)->u8{
	match byte{
		1   => 0, //2^0
		2   => 1, //2^1
		4   => 2, //2^2
		8   => 3, //2^3
		16  => 4, //2^4
		32  => 5, //2^5
		64  => 6, //2^6
		128 => 7, //2^7
		_ => panic!("byte should only contain 1 set bit")
	}
}

pub fn index_to_xyz(lod:u8, idx:u64)->(u64, u64, u64){
	let limit = 1 << lod;
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

