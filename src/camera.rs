use point::Point;

///
/// This is a representation of Camera relative to the virtual world.
///
///
pub struct Camera{
	pub location:Point,
	pub pitch:f64,
	pub yaw:f64,
	pub roll:f64,
}

impl Camera{
	
	pub fn new(location:Point, pitch:f64, yaw:f64, roll:f64)->Camera{
		Camera{
			location:location,
			pitch:pitch,
			yaw:yaw,
			roll:roll
			}
	}
	
	pub fn default()->Camera{
		Camera{
			location:Point::new(0, 0, 0),
			pitch:0.0,
			yaw:0.0,
			roll:0.0
			}
	}
}


impl Clone for Camera {
    fn clone(&self) -> Camera {
    	Camera{
			location:self.location.clone(),
			pitch:self.pitch,
			yaw:self.yaw,
			roll:self.roll
			}	
	}
}