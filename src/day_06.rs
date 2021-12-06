// Advent of Code 2021
// Day 6 - Lanternfish

// On the SIXTH day of Rustmas, Ferris gave to me
// Six refs a-pointing

use crate::useful::file_to_vec;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn day_06(file: &str, days: u16) -> usize {
	let input = file_to_vec::<String>(file);
	let start_fishies = input[0].split(',').fold(Vec::<i32>::new(), |mut vec, f| { vec.push(f.parse::<i32>().unwrap()); vec });
	
	let mut fishies = HashMap::<i32, usize>::new();
	for fish in &start_fishies {
		*fishies.entry(*fish).or_insert(0) += 1;
	}

	for _ in 0..days {
		let mut new_map = HashMap::<i32, usize>::new();
		let mut spawned = 0;
		for key in fishies.keys() {
			let new_key = *key - 1;
			if new_key < 0 {
				spawned += fishies[key];
			} else {
				new_map.insert(new_key, fishies[key]);
			}
		}

		if spawned > 0 {
			*new_map.entry(8).or_insert(0) += spawned;
			*new_map.entry(6).or_insert(0) += spawned;
		}
		
		fishies = new_map;
	}

	fishies.keys().fold(0, |acc, k| acc + fishies[k])
}

#[test]
fn test_day_06() {
	assert_eq!(day_06("day_06_test.txt", 80), 5934);
	assert_eq!(day_06("day_06_test.txt", 256), 26984457539);
}
