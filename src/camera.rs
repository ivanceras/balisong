use std::num::Float;
use point::Point;

///
/// This is a representation of Camera relative to the virtual world.
///
///
pub struct Camera{
	pub eye:Point,
	pub pitch:f64,
	pub yaw:f64,
	pub roll:f64,
}

impl Camera{
	
	pub fn new(location:Point, pitch:f64, yaw:f64, roll:f64)->Camera{
		Camera{
			eye:location,
			pitch:pitch,
			yaw:yaw,
			roll:roll
			}
	}
	
	pub fn look_at(&mut self, lookat:&Point){
		self.pitch = ((lookat.z - self.eye.z) as f64/(lookat.y - self.eye.y) as f64).atan();//along x
		self.yaw =   ((lookat.x - self.eye.x) as f64/(lookat.y - self.eye.y) as f64).atan();//along y
		println!("camera look at pitch:{} yaw:{}",self.pitch.to_degrees(),self.yaw.to_degrees());
	}
	
	pub fn default()->Camera{
		Camera{
			eye:Point::new(0, 0, 0),
			pitch:0.0,
			yaw:0.0,
			roll:0.0
			}
	}
}


impl Clone for Camera {
    fn clone(&self) -> Camera {
    	Camera{
			eye:self.eye.clone(),
			pitch:self.pitch,
			yaw:self.yaw,
			roll:self.roll
			}	
	}
}