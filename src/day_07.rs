// Advent of Code 2021
// Day 7 - The Treachery of Whales

// On the SEVENTH day of Rustmas, Ferris gave to me
// Seven structs a-spanning

use crate::useful::file_to_vec;

fn sigma(n: u64) -> u64 {
	match n {
		0 => 0,
		_ => n + sigma(n - 1)
	}
}

#[allow(dead_code)]
pub fn day_07(file: &str, expensive_fuel: bool) -> u64 {
	let input = file_to_vec::<String>(file);
	let positions = input[0].split(',').fold(Vec::<i32>::new(), |mut vec, n| { vec.push(n.parse::<i32>().unwrap()); vec });
	
	let min = *positions.iter().min().unwrap();
	let max = *positions.iter().max().unwrap();

	// This is VERY slow on part 2; probably not my best solution
	let mut min_moves = u64::MAX;
	for i in min..=max {
		let moves = positions.iter().fold(0, |acc: u64, p| {
			let diff = (p - i).abs() as u64;
			if !expensive_fuel {
				acc + diff
			} else {
				acc + sigma(diff)
			}
		}) as u64;

		if moves < min_moves {
			min_moves = moves;
		}
	}

	min_moves
}

#[test]
fn test_day_07() {
	assert_eq!(day_07("day_07_test.txt", false), 37);
	assert_eq!(day_07("day_07_test.txt", true), 168);
}
