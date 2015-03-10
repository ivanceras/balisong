//screen.rs
use vector::Vector;

pub struct Screen{
    pub width:i64,
    pub height:i64,
    fd:i64,//distance from the eye to the projection screen
}

impl Screen{
    
    pub fn new(width:i64, height:i64, fd:i64)->Screen{
        Screen{width:width, height:height, fd:fd}
    }
    pub fn at_pixel(&self, px:i64, py:i64)->Vector{
     	let z = self.fd;
    	let x = px - self.width/2;
    	let y = self.height/2 - py;

        let mx = x as f64;
        let my = y as f64;
        let mz = z as f64;
        let v = Vector::new(mx, my, mz);
        v
    }
    
    //at distance 0, 1 pixel = 1 voxel
    //factor 0 lod highest detail
    //factor >0.125 to 0.25,  lod + 3   2^-3 = 0.125
    //factor >0.25 to 0.5,    lod + 2   2^-2 = 0.25
    //factor >0.5 to 1,       lod + 1   2^-1 = 0.5
    //factor >1 to 2,         lod - 0   2^0 = 1
    //factor >2 to 4,         lod - 1   2^1 = 2
    //factor >4 to 8,         lod - 2   2^2 = 4
    //factor >8 to 16,        lod - 3   2^3 = 8
    //factor >16 to 32,       lod - 4   2^4 = 16
    //
    // 2 ^ x = n
    /*
    fn get_lod_at_distance(&self, distance:f64){
        let limit = 1 << self.lod;
        let multiplier = 2.0 * limit as f64/ self.width as f64; //multiplier of width = 2 * voxel size
        //println!("multiplier {}/{} : {}",limit, self.width, multiplier);
        let fov = ((self.width as f64/2.0) / self.fd as f64).atan();
        println!("\nfov {}/{}: {}", self.width/2, self.fd, fov.to_degrees());
        let factor = distance / (multiplier * self.width as f64/2.0) * (fov/2.0).tan();//factor will be used to subtract the lod 
        let exp = factor.log2();
        println!("exponent: {}",exp);
        println!("distance {}: {} = 2 ^ {}, rounded: {}", distance, factor, exp, exp.round());//
        println!("lod: {}",self.lod as i64 - exp.round() as i64);
        let required_lod = (self.lod as i16 - exp.round() as i16)
        println!("required_lod");
    }
    */
    
}


impl Clone for Screen {
    fn clone(&self) -> Screen { Screen{width:self.width, height:self.height, fd:self.fd} }
}
