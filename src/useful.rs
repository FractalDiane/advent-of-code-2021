// Some useful reusable functions

use std::str::FromStr;
use std::fmt::Debug;

pub fn file_to_vec<T>(file: &str) -> Vec<T>
where T: FromStr, <T as FromStr>::Err: Debug {
	use std::fs::File;
	use std::io::{self, BufRead};

	let file = File::open(file).unwrap();
	let lines = io::BufReader::new(file).lines();

	let mut vec = Vec::<T>::new();
	for line in lines {
		vec.push(line.unwrap().parse::<T>().unwrap());
	}

	vec
}

#[macro_export]
macro_rules! bench {
	{$what:block} => {
		{
			let t = std::time::Instant::now();
			$what
			let t2 = std::time::Instant::now() - t;
			println!("-----\nExecuted in {} μs", t2.as_micros());
		}
	};

	{$what:expr} => {
		{
			let t = std::time::Instant::now();
			$what;
			let t2 = std::time::Instant::now() - t;
			println!("-----\nExecuted in {} μs", t2.as_micros());
		}
	};
}
