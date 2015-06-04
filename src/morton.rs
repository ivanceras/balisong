///
/// for 8 BIT use base 1
/// for 64 BIT use base 2
/// 
pub fn morton_to_xyz(base:u8, morton:u64)->(u64, u64, u64){
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;
    for i in 0..base {
        x |= ((morton & ( 1  << 3 * i + 0)) >> ((3 * i) + 0)-i);
        y |= ((morton & ( 1  << 3 * i + 1)) >> ((3 * i) + 1)-i);
        z |= ((morton & ( 1  << 3 * i + 2)) >> ((3 * i) + 2)-i);
    }
    (x, y, z)
}
///
/// for 8 BIT use base 1
/// for 64 BIT use base 2
/// 
pub fn xyz_to_morton(base:u8, x:u64, y:u64, z:u64)->u64{
    let mut answer:u64 = 0;
    for i in 0..base {
        answer |= ((x & (1 << i)) << 2*i) | ((y & (1 << i)) << (2*i + 1)) | ((z & (1 << i)) << (2*i + 2));
    }
    answer
}

#[test]
fn test_morton_8bit(){
    let base = 2;
    let (x,y,z) = (1,1,1);
    let m = xyz_to_morton(base, x,y,z);
    println!("morton: {}",m);
    assert_eq!(m, 7);
    let (x1,y1,z1) = morton_to_xyz(base, m);
    assert_eq!((x,y,z), (x1,y1,z1));
}
#[test]
fn test_morton_8bit_4(){
    let base = 2;
    let (x,y,z) = (1,0,0);
    let m = xyz_to_morton(base, x,y,z);
    println!("morton: {}",m);
    assert_eq!(m, 1);
    let (x1,y1,z1) = morton_to_xyz(base, m);
    assert_eq!((x,y,z), (x1,y1,z1));
}
#[test]
fn test_morton_8bit_5(){
    let base = 2;
    let (x,y,z) = (1,0,1);
    let m = xyz_to_morton(base, x,y,z);
    println!("morton: {}",m);
    assert_eq!(m, 5);
    let (x1,y1,z1) = morton_to_xyz(base, m);
    assert_eq!((x,y,z), (x1,y1,z1));
}
#[test]
fn test_morton_8bit_all(){
    let base:u64 = 2;
    for x in 0..base{
        for y in 0..base{
            for z in 0..base{
                let m = xyz_to_morton(base as u8, x,y,z);
                println!("({},{},{}) = {}", x,y,z,m);
                let (x1,y1,z1) = morton_to_xyz(base as u8, m);
                assert_eq!((x,y,z), (x1,y1,z1));
            }
        }
    }
}

#[test]
fn test_morton_8bit_all_base(){
    let base:u64 = 2;
    for x in 0..base{
        for y in 0..base{
            for z in 0..base{
                let m2 = xyz_to_morton(base as u8, x,y,z);
                let m1 = xyz_to_morton(1, x,y,z);
                println!("({},{},{}) = {}", x,y,z,m2);
                assert_eq!(m1, m2);
                let (x2,y2,z2) = morton_to_xyz(base as u8, m2);
                let (x1,y1,z1) = morton_to_xyz(1, m1);
                assert_eq!((x,y,z), (x2,y2,z2));
                assert_eq!((x2,y2,z2), (x1,y1,z1));
            }
        }
    }
}
#[test]
/// TODO? find out why is it ok for morton decode when bits is greater than the base required
fn test_morton_8bit_all_ok_lesser_base(){
    let base:u64 = 2;
    for x in 0..base{
        for y in 0..base{
            for z in 0..base{
                let m = xyz_to_morton(1, x,y,z);
                println!("({},{},{}) = {}", x,y,z,m);
                let (x1,y1,z1) = morton_to_xyz(1, m);
                assert_eq!((x,y,z), (x1,y1,z1));
            }
        }
    }
}
#[test]
fn test_morton_8bit_all_ok_greater_base(){
    let base:u64 = 2;
    for x in 0..base{
        for y in 0..base{
            for z in 0..base{
                let m = xyz_to_morton(3, x,y,z);
                println!("({},{},{}) = {}", x,y,z,m);
                let (x1,y1,z1) = morton_to_xyz(3, m);
                assert_eq!((x,y,z), (x1,y1,z1));
            }
        }
    }
}
#[test]
#[should_panic]
fn test_morton_8bit_fail(){
    let base = 0;
    let (x,y,z) = (1,1,1);
    let m = xyz_to_morton(base, x,y,z);
    println!("morton: {}",m);
    assert_eq!(m, 7);
    let (x1,y1,z1) = morton_to_xyz(base, m);
    assert_eq!((x,y,z), (x1,y1,z1)); 
}

#[test]
/// base is the cuberoot of bitsize
/// 4^3 = 64
fn test_morton_64bit(){
    let base = 4;
    let (x,y,z) = (3,3,3);
    let m = xyz_to_morton(base, x,y,z);
    println!("morton: {}",m);
    assert_eq!(m, 63);
    let (x1,y1,z1) = morton_to_xyz(base, m);
    assert_eq!((x,y,z), (x1,y1,z1));
}

#[test]
fn test_morton_64bit_all(){
    let base:u64 = 4;
    for x in 0..base{
        for y in 0..base{
            for z in 0..base{
                let m = xyz_to_morton(base as u8, x,y,z);
                println!("({},{},{}) = {}", x,y,z,m);
                let (x1,y1,z1) = morton_to_xyz(base as u8, m);
                assert_eq!((x,y,z), (x1,y1,z1));
            }
        }
    }
}

#[test]
fn test_morton_64bit_all_base(){
    let base:u64 = 4;
    for x in 0..base{
        for y in 0..base{
            for z in 0..base{
                let m4 = xyz_to_morton(base as u8, x,y,z);
                let m3 = xyz_to_morton(3, x,y,z);
                let m2 = xyz_to_morton(2, x,y,z);
                let m1 = xyz_to_morton(1, x,y,z); // not OK
                println!("({},{},{}) = {}", x,y,z,m4);
                assert_eq!(m4, m3);
                assert_eq!(m3, m2);
                assert_eq!(m4, m2);
                let (x4,y4,z4) = morton_to_xyz(base as u8, m4);
                let (x3,y3,z3) = morton_to_xyz(3, m3);
                let (x2,y2,z2) = morton_to_xyz(2, m2);
                let (x1,y1,z1) = morton_to_xyz(1, m1);//<---not OK
                assert_eq!((x,y,z), (x4,y4,z4));
                assert_eq!((x4,y4,z4), (x3,y3,z3));
                assert_eq!((x3,y3,z3), (x2,y2,z2));
                assert_eq!((x4,y4,z4), (x2,y2,z2));
            }
        }
    }
}
#[test]
fn test_morton_64bit_all_ok_lesser_base3(){
    let base:u64 = 4;
    for x in 0..base{
        for y in 0..base{
            for z in 0..base{
                let m = xyz_to_morton(3, x,y,z);
                println!("({},{},{}) = {}", x,y,z,m);
                let (x1,y1,z1) = morton_to_xyz(3, m);
                assert_eq!((x,y,z), (x1,y1,z1));
            }
        }
    }
}


#[test]
fn test_morton_64bit_all_ok_lesser_base2(){
    let base:u64 = 4;
    for x in 0..base{
        for y in 0..base{
            for z in 0..base{
                let m = xyz_to_morton(2, x,y,z);
                println!("({},{},{}) = {}", x,y,z,m);
                let (x1,y1,z1) = morton_to_xyz(2, m);
                assert_eq!((x,y,z), (x1,y1,z1));
            }
        }
    }
}

#[test]
#[should_panic]
/// fails if base is 1
fn test_morton_64bit_all_fail_lesser_base1(){
    let base:u64 = 4;
    for x in 0..base{
        for y in 0..base{
            for z in 0..base{
                let m = xyz_to_morton(1, x,y,z);
                println!("({},{},{}) = {}", x,y,z,m);
                let (x1,y1,z1) = morton_to_xyz(1, m);
                assert_eq!((x,y,z), (x1,y1,z1));
            }
        }
    }
}

#[test]
/// morton is ok with any base as long as it is more than the required base cuberoot of bit_size
/// 4^3 = 64
fn test_morton_64bit_ok(){
    let base = 6;
    let (x,y,z) = (3,3,3);
    let m = xyz_to_morton(base, x,y,z);
    println!("morton: {}",m);
    assert_eq!(m, 63);
    let (x1,y1,z1) = morton_to_xyz(base, m);
    assert_eq!((x,y,z), (x1,y1,z1));
}

#[test]
#[should_panic]
fn test_morton_64bit_fail(){
    let (x,y,z) = (3,3,3);
    let m = xyz_to_morton(1, x,y,z);
    println!("morton: {}",m);
    assert_eq!(m, 63);
}