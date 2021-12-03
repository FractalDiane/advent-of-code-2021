// Advent of Code 2021
// Day 2 - Dive!

// On the SECOND day of Rustmas, Ferris gave to me
// Two closure pipes ||

use crate::useful::file_to_vec;

#[allow(dead_code)]
pub fn day_02(file: &str) -> i32 {
	let vec = file_to_vec::<String>(file);

	let mut hpos = 0;
	let mut depth = 0;
	let mut aim = 0;

	for command in vec {
		let words: Vec<&str> = command.split_whitespace().collect();
		let amount = words[1].parse::<i32>().unwrap();
		match words[0] {
			"forward" => {
				hpos += amount;
				depth += aim * amount;
			},
			"down" => {
				aim += amount;
			},
			_ => {
				aim -= amount;
			},
		}
	}

	hpos * depth
}

#[test]
fn test_day_02() {
	assert_eq!(day_02("day_02_test.txt"), 900);
}
