/// linear arrangement of heirarchial voxels to efficiently reduce memory footprint by arranging the children voxels in a vector
/// The weakness of octree and voxtree was the pointer to each children. The pointer is 64 bits, while it points to an 8bit or a 64bit value of
/// the underlying child node, it has an added memory consumption of 100% as much as the data it points to.
use lod::LOD;
use bitset;

pub struct Voxline{
    pub lod:LOD,
    pub line:Vec<Vec<u64>>,
}


impl Voxline{
    
    pub fn new(lod:LOD)->Voxline{
        let levels = lod.lod as usize;
        let mut line = Vec::new();
        for i in 0..levels{
            line.push(Vec::new());
        }
        Voxline{lod:lod, line:line}
    }
    
    pub fn set_voxel(&mut self, location:&Vec<u64>){
        for i in 0..location.len(){
            let offset = self.get_offset(i, location);
            self.line[i][offset] = self.line[i][offset] | location[i]
        }
    }
    
    /// return the offset of the 64bit integer at this location
    /// if i == 0,  0
    /// else count the number of bits from the previous level
    /// the prev_level_cnt * 64 + lowest_bitset(location) * 64
    pub fn get_offset(&self, level:usize, location:&Vec<u64>)->usize{
        for i in 0..level{
            let bitsets = Voxline::count_setbits(&self.line[i]);
            println!("bitsets: {}",bitsets);
        }
        42
    }
    
    pub fn count_setbits(bitset:&Vec<u64>)->usize{
        let mut cnt = 0;
        for i in 0..bitset.len(){
            cnt += bitset::fast_count(bitset[i]);
        }
        cnt
    }
}