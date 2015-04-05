extern crate balisong;
extern crate time;
use balisong::bitset;
use time::PreciseTime;

fn main(){
	println!("count: {}", bitset::count(5));
	println!("fast count: {}", bitset::fast_count(5));
	let limit = 1 << 32;
	println!("limit: {}", limit);
	let start = PreciseTime::now();
	for i in 0..limit{
		let setbits = bitset::count(i);
		let fastsetbits = bitset::fast_count(i);
		assert!(setbits == fastsetbits);
	}
	let both_duration = start.to(PreciseTime::now());
	println!("checking duration: {} ms",both_duration.num_milliseconds());
	let start = PreciseTime::now();
	for i in 0..limit{
		let setbits = bitset::count(i);
	}
	let count_duration = start.to(PreciseTime::now());
	let start = PreciseTime::now();
	for i in 0..limit{
		let setbits = bitset::fast_count(i);
	}
	let fast_count_duration = start.to(PreciseTime::now());
	println!("duration count: {} ms fast_count:{} ms",count_duration.num_milliseconds(), fast_count_duration.num_milliseconds());
}

