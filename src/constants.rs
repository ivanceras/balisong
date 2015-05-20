//or calculate only when point is traced and occupied
pub static PRECALCULATE_NORMALS:bool = true;
pub static USE_GAMMA_CORRECTION:bool = false;
pub static SMOOTHEN_NORMALS:bool = true;
pub static DO_CELLSHADING:bool = false;
pub static SMOOTHING_ITERATION:usize = 1;
//neighbor radius for extraction of neighboring voxels
pub static NEIGHBOR_RADIUS:f64 = 1.0;

//for 64 bits
//pub static BITS:u8 = 64;
//pub static MAX_LOD:u8 = 9;
//pub static BASE:u8 = 4; 

//for 8 bits
pub static BITS:u8 = 8;
pub static MAX_LOD:u8 = 21;
pub static BASE:u8 = 2; 