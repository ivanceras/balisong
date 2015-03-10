use color::Color;
use normal::Normal;
use std::fmt;

pub struct Voxel{
	pub color: Color,//opacity is in Alpha channel
	normal: Normal,
	specularity:u8,//maybe the same as reflectivity
	refraction: u8,
	//density:u8, 8 bit density(can be 16bit, u16) of the voxel 255 = all occupied, 0 = not occupied, 128 halfly occupied
}

impl Voxel{

	pub fn new(color:Color, normal:Normal, specularity:u8, refraction:u8)->Voxel{
		Voxel{color:color, normal:normal, specularity:specularity, refraction:refraction}
	}	

}


impl Clone for Voxel {
    fn clone(&self) -> Voxel { 
    	Voxel{	color:self.color.clone(),
    			normal:self.normal.clone(),
    			specularity:self.specularity,
    			refraction:self.refraction
    		} 
	}
}

impl fmt::Display for Voxel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(color: {}, normal:{}, specularity:{}, refraction:{})", self.color, self.normal, self.specularity, self.refraction)
    }
}