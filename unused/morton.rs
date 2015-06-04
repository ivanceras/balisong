//morton.rs
use std::num::Float;

pub struct Morton{
	lod:u8,
	pub limit:u64,
	pub size:u64,
}

impl Morton{
	pub fn new(lod:u8)->Morton{
		let limit = 1 << lod; //2^lod
		let size = limit * limit * limit;
		Morton{lod:lod, limit:limit, size: size}
	}	

	pub fn encode(&self, x:u64, y:u64, z:u64)->u64{
		let mut answer:u64 = 0;
		for i in 0..self.lod {
			answer |= ((x & (1 << i)) << 2*i) | ((y & (1 << i)) << (2*i + 1)) | ((z & (1 << i)) << (2*i + 2));
		}
		answer
	}
	
	// decode a given 64-bit morton code to an integer (x,y,z) coordinate
	pub fn decode(&self, morton:u64)->(u64, u64, u64){
		let mut x = 0;
		let mut y = 0;
		let mut z = 0;
		for i in 0..self.lod {
			x |= ((morton & ( 1  << 3 * i + 0)) >> ((3 * i) + 0)-i);
			y |= ((morton & ( 1  << 3 * i + 1)) >> ((3 * i) + 1)-i);
			z |= ((morton & ( 1  << 3 * i + 2)) >> ((3 * i) + 2)-i);
		}
		(x, y, z)
	}
	
	
	pub fn a_gt_b(&self, a:u64, b:u64)->bool{
		let am = self.decode(a);
		let bm = self.decode(b);
		am > bm
	}
	
	//get the morton index at this LOD level
	pub fn at_lod(&self, m:u64, new_lod:u8)->u64{
		let div = 1 << (3 * (self.lod - new_lod));
		let new_m = m/div;//original
		new_m
	}
	
	//wont be accurate due to rounding/flooring of value
	pub fn point_at_lod(&self, x:u64, y:u64, z:u64, new_lod:u8)->(u64, u64, u64){
		let m = self.encode(x, y, z);
		let new_m = self.at_lod(m, new_lod);
		let new_morton = Morton::new(new_lod);
		let (xnew, ynew, znew) = new_morton.decode(new_m);
		(xnew, ynew, znew)
	}
	
	
	 pub fn point_at_lod_rounded(&self, x:u64, y:u64, z:u64, new_lod:u8)->(u64, u64, u64){
    	let div = 1 << (self.lod - new_lod);//same as (1 << self.lod) / (1 << new_lod)
    	let xnew = (x as f64 / div as f64).round() as u64;
    	let ynew = (y as f64 / div as f64).round() as u64;
    	let znew = (z as f64 / div as f64).round() as u64;
    	(xnew, ynew, znew)
    }
	
	pub fn to_linear(&self, x:u64, y:u64, z:u64)->u64{
		x * self.limit * self.limit + y * self.limit + z
	}
	
	pub fn morton_to_linear(&self, morton:u64)->u64{
		let (x, y, z) = self.decode(morton);
		self.to_linear(x, y, z)
	}
	
	//from linear index to x,y,z
	pub fn linear_to_coord(&self, idx:u64)->(u64, u64, u64){
	    let mut index = idx;
	    let z =  index % self.limit;
	    index /= self.limit;
	    let y = index % self.limit;
	    index /= self.limit;
	    let x = index;
	    return (x,y,z)
	}

}

