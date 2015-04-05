
/// count the number of bits
/// http://stackoverflow.com/questions/109023/how-to-count-the-number-of-set-bits-in-a-32-bit-integer
pub fn fast_count(i:u64)->u8{
	 let i = i - ((i >> 1) & 0x55555555);
     let i = (i & 0x33333333) + ((i >> 2) & 0x33333333);
     let result =  (((i + (i >> 4)) & 0x0F0F0F0F) * 0x01010101) >> 24;
     result as u8
}


pub fn count(arg:u64)->u8 {
    let mut count:u8 = 0;
    let mut x = arg;
    while x > 0 {
        x &= x-1;
        count += 1;
    }
    count
}


