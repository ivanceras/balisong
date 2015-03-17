use std::num::Float;

fn main(){
	let cores = 8;
	let width = 7;
	let height = 6;
	
	let size = width * height;
	
	println!("size: {}",size);
	println!("parts: {} + {}", size/cores, size%cores);
	
	let parts = (size as f64 / cores as f64).ceil() as u64;
	println!("actual parts: {}", parts);
	let rem = size % cores;
	for i in 0..cores{
		let start = i * parts;
		let end = (i+1)* parts;
		println!("batch: {} = {} to {}",i , start, end);
		for j in start..end{
			if j > size-1 {
				break;
			}
			println!("\tdoing: {}",j);
		}
	}
}