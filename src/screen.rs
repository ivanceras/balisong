//screen.rs

use vector::Vector;
use lod::LOD;
use constants;

pub struct Screen{
    pub width:i64,
    pub height:i64,
    pub fd:i64,//distance from the eye to the projection screen
    pub fov:f64,//angle view in radians
}

impl Screen{
    
    pub fn new(width:i64, height:i64, fd:i64)->Screen{
    	let fov = 2.0 * ( width as f64 / 2.0 / fd as f64).atan();
        Screen{width:width, height:height, fd:fd, fov:fov}
    }
    pub fn at_pixel_up_y(&self, px:i64, py:i64)->Vector{
     	let z = self.fd;
    	let x = px - self.width/2;
    	let y = self.height/2 - py;

        let mx = x as f64;
        let my = y as f64;
        let mz = z as f64;
        let v = Vector::new(mx, my, mz);
        v
    }
    
    //using z as the up, y forward to screen, x to the right of the screen
    pub fn at_pixel(&self, px:i64, py:i64)->Vector{
     	let y = self.fd;
    	let x = px - self.width/2;
    	let z = self.height/2 - py;

        let mx = x as f64;
        let my = y as f64;
        let mz = z as f64;
        let v = Vector::new(mx, my, mz);
        v
    }

	//calculate the view LOD resolution base on screen size
	//find the LOD such that 1 << LOD > = screen.width
	pub fn get_view_lod(&self)->LOD{
		let lod = (self.width as f64).log(constants::BASE as f64).ceil() as u8;//ceil for best view, round() for better performace
		LOD::new(lod)
	}
	
	//calculate the required LOD to load for an object at certain distance
	
	pub fn get_required_lod(&self, view_lod:u8, distance:f64)->i8{
	    let view_limit = 1 << view_lod;
	    let factor = distance / view_limit as f64 * (self.fov/2.0).tan();//factor will be used to subtract the lod 
	    let exp = factor.log(constants::BASE as f64).round();
	    let required_lod = (view_lod as i16 - exp as i16) as i8;
	    required_lod
	}    
}


impl Clone for Screen {
    fn clone(&self) -> Screen { 
    	Screen{
    		width:self.width, 
    		height:self.height, 
    		fd:self.fd,
    		fov:self.fov
		} 
	}
}
