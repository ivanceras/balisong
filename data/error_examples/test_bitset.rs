extern crate balisong;
extern crate time;
use balisong::bitset;
use time::PreciseTime;

fn main(){
	let n = 12345790;
	let bits = bitset::lowest_bitset(n);
	println!("lowest bit of {} is {}", n, bits);
	
	let powerof2 = bitset::is_power_of_2(n);
	println!("is {} power of 2 {}",n, powerof2);
	
	println!("it has {} set bits or {}",bitset::fast_count(n), bitset::count(n));
}