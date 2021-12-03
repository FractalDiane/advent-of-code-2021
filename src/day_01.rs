// Advent of Code 2021
// Day 1 - Sonar Sweep

// On the FIRST day of Rustmas, Ferris gave to me
// A lack of a segfault I'd have had in C

use crate::useful::file_to_vec;

#[allow(dead_code)]
pub fn day_01(file: &str) -> u32 {
	let vec = file_to_vec::<u32>(file);
	
	let mut count = 0u32;
	let mut last_sum = 0u32;
	for i in 0..vec.len() - 2 {
		let sum = vec[i] + vec[i + 1] + vec[i + 2];
		if i > 0 && sum > last_sum {
			count += 1;
		}

		last_sum = sum;
	}

	count
}

#[test]
fn test_day_01() {
	assert_eq!(day_01("day_01_test.txt"), 5);
}
