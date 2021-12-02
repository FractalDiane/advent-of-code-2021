use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;
use std::fmt::Debug;

pub fn file_to_vec<T>(file: &str) -> Vec<T>
where T: FromStr, <T as FromStr>::Err: Debug {
	let file = File::open(file).unwrap();
	let lines = io::BufReader::new(file).lines();

	let mut vec = Vec::<T>::new();
	for line in lines {
		vec.push(line.unwrap().parse::<T>().unwrap());
	}

	vec
}
