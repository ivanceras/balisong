use std::num::Int;

fn main(){
    
    //let limit = 1 << lod; //same as 2^lod , volume = (2^lod)^3 at lod 1, limit = 2, volume = 8
    //for 64 volumes same as 4^lod , volume = (4^lod)^3 at lod 1, limit = 4, volume = 64
    for i in 0..64{
        let lod = i;
        let limit = 4u64.pow(lod+1);
        let volume =  limit * limit * limit;
        println!("lod: {} limit: {}, volume:{} ",lod, limit, volume);
    }
}