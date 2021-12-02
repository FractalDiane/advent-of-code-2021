use std::fs::File;
use std::io::{self, BufRead};

#[allow(unused)]
pub fn day01() {
	let file = File::open("day01.txt").unwrap();
	let lines = io::BufReader::new(file).lines();

	let mut vec = Vec::<u32>::new();
	for line in lines {
		vec.push(line.unwrap().parse::<u32>().unwrap());
	}
	
	let mut count = 0;
	let mut last_sum = 0u32;
	for i in 0..vec.len() - 2 {
		let sum = vec[i] + vec[i + 1] + vec[i + 2];
		if i > 0 && sum > last_sum {
			count += 1;
		}

		last_sum = sum;
	}

	println!("{}", count);
}
