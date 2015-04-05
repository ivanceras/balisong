//computes the limit and volumes of based on LOD
//lod can not be greater than 9

use std::num::Float;
use std::fmt;

use constants;

pub struct LOD{
	pub lod:u8,
	pub limit:u32,
	pub volume:u64,
}

impl LOD{
	pub fn new(lod:u8)->LOD{
		if lod > constants::MAX_LOD{
			panic!("LOD can not be greater than {}", constants::MAX_LOD);
		}
		let base = constants::BASE;
		let limit = (base as f64).powf(lod as f64) as u32;
		println!("limit: {}", limit);
		let volume =  limit as u64 * limit as u64 * limit as u64;
		LOD{lod:lod, limit:limit, volume:volume}
	}
	
	pub fn from_volume(volume:u64)->LOD{
		let limit = (volume as f64).cbrt();
		let lod = limit.log(constants::BASE as f64) as u8;
		LOD::new(lod)
	}
}

impl Clone for LOD {
    fn clone(&self) -> LOD { 
    	LOD{
    		lod:self.lod, 
    		limit:self.limit, 
    		volume:self.volume,
		} 
	}
}


impl fmt::Display for LOD {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.lod)
    }
}