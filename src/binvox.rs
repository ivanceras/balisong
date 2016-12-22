extern crate regex;

use std::path::Path;
use std::fs::File;
use point::Point;
use vector::Vector;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;
use color::Color;
use location;
use voxel::voxtree::Voxtree;
use voxel::voxbit::Voxbit;
use voxel::vox::Vox;
use normal::Normal;
use voxelizer;
use lod::LOD;
use constants;

pub struct Binvox{
    version:String,
    dim:Point,
    translate:Vector,
    scale:f64
}

impl Binvox{
    
    pub fn read_file(filename:String)->(LOD, Voxtree<Normal>){
        let path = Path::new(&filename);
        let display = path.display();    
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}", display),
            Ok(file) => file,
        };
        
        let mut reader = BufReader::new(file);

        //read header version    
        let version = read_header(&mut reader);
        let (xlimit, ylimit, zlimit) = read_dim(&mut reader);
        let (xtrans, ytrans, ztrans) = read_translation(&mut reader);    
        let scale = read_scaling(&mut reader);    
        let size = xlimit * ylimit * zlimit;
        println!("size: {}", size);
        let normals = read_data(&mut reader, size);
        
        let binvox = Binvox{
                    version: version, 
                    dim: Point::new(xlimit as i64, ylimit as i64, zlimit as i64),
                    translate: Vector::new(xtrans, ytrans, ztrans),
                    scale: scale
                };
        let size = xlimit * ylimit * zlimit;
        let lod = LOD::from_volume(size);
        
        (lod, normals)
                
    }

}


fn read_header(reader:&mut BufRead)->String{
        //read header version
    let mut buff = String::new();
    let mut line = match reader.read_line(&mut buff) {
        Err(why) => panic!("error reading header"),
        Ok(string) => string,
    };
    let re = match regex::Regex::new(r"^(#binvox) (\d+)") {
        Ok(re) => re,
        Err(err) => panic!("{}", err),
    };
    if re.is_match(&buff){
        let cap = re.captures(&buff).unwrap();
        let version = cap.at(2).unwrap();
        return format!("{}",version);
    }
    else{
        panic!("invalid binvox format at binvox!");
    }
}

fn read_dim(reader:&mut BufRead)->(u64, u64, u64){
    let mut buff = String::new();
    let mut line = match reader.read_line(&mut buff) {
        Err(why) => panic!("couldn't read dimension"),
        Ok(string) => string,
    };
    
    let re = match regex::Regex::new(r"^(dim)\s+(\d+)\s+(\d+)\s+(\d+)") {
        Ok(re) => re,
        Err(err) => panic!("{}", err),
    };
    if re.is_match(&buff){
        let cap = re.captures(&buff).unwrap();
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

fn read_translation(reader:&mut BufRead)->(f64, f64, f64){
    let mut buff = String::new();
    let mut line = match reader.read_line(&mut buff) {
        Err(why) => panic!("couldn't read translation"),
        Ok(string) => string,
    };
    
    let re = match regex::Regex::new(r"^(translate)\s+([+-]?\d+\.\d+)\s+([+-]?\d+\.\d+)\s+([+-]?\d+\.\d+)") {
        Ok(re) => re,
        Err(err) => panic!("{}", err),
    };
    if re.is_match(&buff){
        let cap = re.captures(&buff).unwrap();
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

fn read_scaling(reader:&mut BufRead)->f64{
    let mut buff = String::new();
    let mut line = match reader.read_line(&mut buff) {
        Err(why) => panic!("couldn't read scaling"),
        Ok(string) => string,
    };
    
    let re = match regex::Regex::new(r"^(scale)\s+(.*)") {
        Ok(re) => re,
        Err(err) => panic!("{}", err),
    };
    if re.is_match(&buff){
        let cap = re.captures(&buff).unwrap();
        let scale = cap.at(2).unwrap();
        println!("scale: {}",scale);
        return f64::from_str(scale).unwrap();
    }
    else{
        panic!("invalid binvox format at scale!");
    }
}

fn read_data(reader:&mut BufRead, size:u64)->Voxtree<Normal>{
    
    let lod = LOD::from_volume(size);
    let mut buff = String::new();
    let mut line = match reader.read_line(&mut buff) {
        Err(why) => panic!("couldn't read data"),
        Ok(string) => string,
    };
    
    let re = match regex::Regex::new(r"^(data)\s+(.*)") {
        Ok(re) => re,
        Err(err) => panic!("{}", err),
    };
    if re.is_match(&buff){
        let cap = re.captures(&buff).unwrap();
        let data = cap.at(1).unwrap();
        println!("data: {}",data);
        
        let mut end_index = 0u64;
        let mut nr_voxels = 0u64;
        let mut index = 0u64;
        let mut linear_voxels = Vec::new();
        let mut buff:Vec<u8> = Vec::new();
        reader.read_to_end(&mut buff);
        let mut i = 0;
        while i < buff.len() {
                let value = buff[i];
                let count = buff[i+1];
                //println!("value: {}, count:{}", value, count);
                end_index = index + count as u64;
                if end_index > size {break;}
                for j in index..end_index {
                    linear_voxels.push(value);
                }
                if value > 0 {nr_voxels += count as u64;}    
                index = end_index;
                i+=2;
        }
        
        println!("There are {} voxels",linear_voxels.len());
        let mut cnt = 0;
        let mut root = Voxbit::new();
        println!("loading binvox....");
        let mut percentage = 0;
        let mut index = 0;
        for value in &linear_voxels{
            let new_percentage = ((index as f64 / linear_voxels.len() as f64) * 100.0).round() as u64;
            if percentage != new_percentage {
                println!("{}%",new_percentage);
            }
            percentage = new_percentage;
            if *value > 0 {//no carving
                let (x,y,z) = location::index_to_xyz(&lod, index as u64);
                let loc =  location::from_xyz(&lod, x, y, z);
                root.set_location(&loc);
                cnt += 1;
            }
            index += 1;
        }
        println!("before shrink leaves: {}",root.count_leaves());
        let mut normals = Voxtree::new();
        if constants::PRECALCULATE_NORMALS{
            normals = voxelizer::calculate_normals(&root, &lod);
        }
        if constants::SMOOTHEN_NORMALS{
            for k in 0..constants::SMOOTHING_ITERATION{
                println!("Pass {}.. ",k);
                normals = voxelizer::smoothen_normals(&root, &normals, &lod);
            }
        }
        //calculate lower normals
        for l in 1..lod.lod{
            let lod1 = lod.lod - l;// if lod = 5, then 4,3,2,1
            let lower_lod = LOD::new(lod1); 
            println!("Calculating normals at LOD: {} ",lod1);
            //normals = voxelizer::calculate_lower_lod_normals(&root, normals, &lower_lod);
            let orig_normals = normals.clone();
            normals = voxelizer::calculate_lower_lod_normals_using_average(&root, normals, &orig_normals, &lower_lod);
        }
        return normals;
        
    }
    else{
        panic!("invalid binvox format at data!");
    }    
}

