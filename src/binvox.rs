extern crate regex;

use std::old_io::File;
use point::Point;
use vector::Vector;
use std::old_io::BufferedReader;
use std::str::FromStr;
use std::num::Float;
use color::Color;
use voxel::Voxel;
use location;
use octree::Octree;
use normal::Normal;

pub struct Binvox{
	version:String,
	dim:Point,
	translate:Vector,
	scale:f64
}

impl Binvox{
	
	pub fn read_file(filename:&'static str)->(u8, Octree){
		let path = Path::new(filename);
    	let display = path.display();	
	    let mut file = match File::open(&path) {
	        Err(why) => panic!("couldn't open {}: {}", display, why.desc),
	        Ok(file) => file,
	    };
	    
	    let mut reader = BufferedReader::new(file);

		//read header version	
		let version = read_header(&mut reader);
		let (xlimit, ylimit, zlimit) = read_dim(&mut reader);
		let (xtrans, ytrans, ztrans) = read_translation(&mut reader);	
		let scale = read_scaling(&mut reader);	
		let size = xlimit * ylimit * zlimit;
		println!("size: {}", size);
		let octree = read_data(&mut reader, size);
		
		let binvox = Binvox{
					version: version, 
					dim: Point::new(xlimit as i64, ylimit as i64, zlimit as i64),
					translate: Vector::new(xtrans, ytrans, ztrans),
					scale: scale
				};
		let size = xlimit * ylimit * zlimit;
		let lod = lod_from_size(size);
		
		(lod, octree)
				
	}

}


fn read_header(reader:&mut BufferedReader<File>)->String{
		//read header version		
    let mut line = match reader.read_line() {
        Err(why) => panic!("error reading header"),
        Ok(string) => string,
    };
	let re = match regex::Regex::new(r"^(#binvox) (\d+)") {
	    Ok(re) => re,
	    Err(err) => panic!("{}", err),
	};
	if re.is_match(line.as_slice()){
		let cap = re.captures(line.as_slice()).unwrap();
		let version = cap.at(2).unwrap();
		println!("version: {}",version);
		return String::from_str(version);
	}
	else{
		panic!("invalid binvox format at binvox!");
	}
}

fn read_dim(reader:&mut BufferedReader<File>)->(u64, u64, u64){
	let mut line = match reader.read_line() {
        Err(why) => panic!("couldn't read dimension"),
        Ok(string) => string,
    };
    
	let re = match regex::Regex::new(r"^(dim)\s+(\d+)\s+(\d+)\s+(\d+)") {
	    Ok(re) => re,
	    Err(err) => panic!("{}", err),
	};
	if re.is_match(line.as_slice()){
		let cap = re.captures(line.as_slice()).unwrap();
		let dimx = cap.at(2).unwrap();
		let dimy = cap.at(3).unwrap();
		let dimz = cap.at(4).unwrap();
		let xlimit = u64::from_str(dimx).unwrap();
		let ylimit = u64::from_str(dimy).unwrap();
		let zlimit = u64::from_str(dimz).unwrap();
		println!("limit: {} {} {}",xlimit, ylimit, zlimit);
		println!("dim: {}, {}, {}",dimx, dimy, dimz);
		return (xlimit, ylimit, zlimit);
	}
	else{
		panic!("invalid binvox format at dim!");
	}
}

fn read_translation(reader:&mut BufferedReader<File>)->(f64, f64, f64){
	let mut line = match reader.read_line() {
        Err(why) => panic!("couldn't read translation"),
        Ok(string) => string,
    };
    
	let re = match regex::Regex::new(r"^(translate)\s+([+-]?\d+\.\d+)\s+([+-]?\d+\.\d+)\s+([+-]?\d+\.\d+)") {
	    Ok(re) => re,
	    Err(err) => panic!("{}", err),
	};
	if re.is_match(line.as_slice()){
		let cap = re.captures(line.as_slice()).unwrap();
		for i in 0..cap.len(){
			println!("cap: {}",cap.at(i).unwrap());
		}
		let xtmp = cap.at(2).unwrap();
		let ytmp = cap.at(3).unwrap();
		let ztmp = cap.at(4).unwrap();
		println!("translate: {}, {}, {}",xtmp, ytmp, ztmp);
		let xtrans = f64::from_str(xtmp).unwrap();
		let ytrans = f64::from_str(ytmp).unwrap();
		let ztrans = f64::from_str(ztmp).unwrap();

		return (xtrans, ytrans, ztrans)
	}
	else{
		panic!("invalid binvox format at translate!");
	}	
}

fn read_scaling(reader:&mut BufferedReader<File>)->f64{
	let mut line = match reader.read_line() {
        Err(why) => panic!("couldn't read scaling"),
        Ok(string) => string,
    };
    
	let re = match regex::Regex::new(r"^(scale)\s+(.*)") {
	    Ok(re) => re,
	    Err(err) => panic!("{}", err),
	};
	if re.is_match(line.as_slice()){
		let cap = re.captures(line.as_slice()).unwrap();
		let scale = cap.at(2).unwrap();
		println!("scale: {}",scale);
		return f64::from_str(scale).unwrap();
	}
	else{
		panic!("invalid binvox format at scale!");
	}
}

fn read_data(reader:&mut BufferedReader<File>, size:u64)->Octree{
	
	let lod = lod_from_size(size);
	println!("lod: {}",lod);
	let mut line = match reader.read_line() {
        Err(why) => panic!("couldn't read data"),
        Ok(string) => string,
    };
    
	let re = match regex::Regex::new(r"^(data)\s+(.*)") {
	    Ok(re) => re,
	    Err(err) => panic!("{}", err),
	};
	if re.is_match(line.as_slice()){
		let cap = re.captures(line.as_slice()).unwrap();
		let data = cap.at(1).unwrap();
		println!("data: {}",data);
		
		let mut end_index = 0u64;
		let mut nr_voxels = 0;
		let mut index = 0u64;
		let mut linear_voxels = Vec::new();
		while end_index < size {
				let value = reader.read_u8().unwrap();
				let count = reader.read_u8().unwrap();
				end_index = index + count as u64;
				if end_index > size {break;}
				for i in index..end_index {
					linear_voxels.push(value);
				}
				if value > 0 {nr_voxels += count;}	
				index = end_index;
		}
		
		println!("There are {} voxels",linear_voxels.len());
		let mut root = Octree::new();
		println!("loading binvox....");
		let mut percentage = 0;
		for i in 0..linear_voxels.len(){
			let new_percentage = ((i as f64 / linear_voxels.len() as f64) * 100.0).round() as u64;
			if percentage != new_percentage {
				println!("{}%",new_percentage);
			}
			percentage = new_percentage;
			let value = linear_voxels[i];
			let is_last2_empty = if i > 1 {linear_voxels[i - 2] == 0}else{ false };
			let is_last1_empty = if i > 0 {linear_voxels[i - 1] == 0}else{ false };
			let is_next1_empty = if i < linear_voxels.len() -1 {linear_voxels[i + 1] == 0 } else { false };
			let is_next2_empty = if i < linear_voxels.len() -2 {linear_voxels[i + 2] == 0 } else { false };
			//TODO: checking is on the neighboring voxel, there are 16 neighbors
			//carving enabled to save a lot of memory
			//optimize, only when it changes from 0 to 1 or 1 to zero, store the tree
			//if value > 0 && (is_next2_empty || is_next1_empty || is_last1_empty || is_last2_empty ){
			//if value > 0 && (is_next2_empty || is_last2_empty ){
			//if value > 0 && (is_next1_empty || is_last1_empty ){
			if value > 0 {//no carving
				let (x,y,z) = location::index_to_xyz(lod, i as u64);
				//let (x,y,z) = location::index_to_zyx(lod, i as u64);
				//let (x,y,z) = location::index_to_yzx(lod, i as u64);
				//let (x,y,z) = location::index_to_xzy(lod, i as u64);
				let loc =  location::from_xyz(lod, x, y, z);
				root.set_tree(loc);
			}
		}
		return root;
		
	}
	else{
		panic!("invalid binvox format at data!");
	}	
}

fn lod_from_size(size:u64)->u8{
	let limit = (size as f64).cbrt();
	let lod = limit.log2();
	lod as u8
}