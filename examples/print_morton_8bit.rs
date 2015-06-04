extern crate balisong;
use balisong::morton;

fn main(){
    let base:u64 = 2;
    let mut index = 0;
    for x in 0..base{
        for y in 0..base{
            for z in 0..base{
                let m = morton::xyz_to_morton(base as u8, x,y,z);
                println!("[{}] ({},{},{}) = {}",index, x,y,z,m);
                index += 1;
            }
        }
    }
}