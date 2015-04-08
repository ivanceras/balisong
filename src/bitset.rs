use constants;

/// count the number of bits
/// http://stackoverflow.com/questions/109023/how-to-count-the-number-of-set-bits-in-a-32-bit-integer
/// http://stackoverflow.com/questions/2709430/count-number-of-bits-in-a-64-bit-long-big-integer
/// for 64 bits
pub fn fast_count(i:u64)->usize{
    let i = i - ((i >> 1) & 0x5555555555555555);
    let i = (i & 0x3333333333333333) + ((i >> 2) & 0x3333333333333333);
    let r = (((i + (i >> 4)) & 0xF0F0F0F0F0F0F0F) * 0x101010101010101) >> 56;
    r as usize
}


pub fn count(arg:u64)->usize{
    let mut count = 0;
    let mut x = arg;
    while x > 0 {
        x &= x-1;
        count += 1;
    }
    count
}

//get at which bit that is 1.
pub fn index_of(bitset:u64, location:u64)->usize{
	let mut index = 0;
	for i in 0..constants::BITS{
		let byte = 1 << i;
		if byte == location {
			return index;
		}
		if bitset & byte == byte{
			index += 1;
		}
	}
	return index;
}

/// http://bits.stephan-brumme.com/lowestBitSet.html
pub fn lowest_bitset(x:u64)->usize{
	let mut x = x;
	if x == 0{
		return 0;
 	}
 	let mut result = 0;
	while (x & 1) == 0{
     	x >>= 1;
     	result += 1;
   	}
 	return result;
}

/// http://bits.stephan-brumme.com/isPowerOfTwo.html
pub fn is_power_of_2(x:u64)->bool{
   return ((x & (x - 1)) == 0);
}

