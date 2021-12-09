// Advent of Code 2021
// Day 8 - Seven Segment Search

// On the EIGHTH day of Rustmas, Ferris gave to me
// Eight crates a-scoping

use crate::useful::file_to_vec;

#[allow(dead_code)]
pub fn day_08(file: &str) -> u32 {
	let input = file_to_vec::<String>(file);

	let mut simple_count = 0u32;
	for note in input {
		let mut split = note.split(" | ");
		let output = split.nth(1).unwrap().split_whitespace().collect::<Vec<&str>>();

		for digit in output {
			let len = digit.len();
			if len != 6 && len != 5 {
				simple_count += 1;
			}
		}
	}

	simple_count
}

use std::{collections::HashSet, iter::FromIterator};

#[allow(dead_code)]
pub fn day_08_part2(file: &str) -> u32 {
	let input = file_to_vec::<String>(file);

	let mut total_output = 0u32;
	for note in input {
		let mut split = note.split(" | ");
		let signals = split.next().unwrap().split_whitespace().collect::<Vec<&str>>();
		let output = split.next().unwrap().split_whitespace().collect::<Vec<&str>>();

		let mut patterns = vec![HashSet::<char>::new(); 10];
		
		const SIGNAL_VALUES: [usize; 8] = [0, 0, 1, 7, 4, 0, 0, 8];
		signals.iter().filter(|s| s.len() != 6 && s.len() != 5).for_each(|s| {
			patterns[SIGNAL_VALUES[s.len() as usize]] = HashSet::from_iter(s.chars());
		});

		let mut twofive = Vec::<&str>::new();
		signals.iter().filter(|s| s.len() == 6 || s.len() == 5).for_each(|s| {
			let set = HashSet::<char>::from_iter(s.chars());

			if set.is_superset(&patterns[4]) {
				patterns[9] = set;
			} else if set.is_superset(&patterns[7]) {
				let digit = if s.len() == 5 { 3 } else { 0 };
				patterns[digit] = set;
			} else if s.len() == 6 {
				patterns[6] = set;
			} else {
				twofive.push(s);
			}
		});

		twofive.into_iter().for_each(|s| {
			let set = HashSet::<char>::from_iter(s.chars());
			if set.is_subset(&patterns[9]) {
				patterns[5] = set;
			} else {
				patterns[2] = set;
			}
		});

		let mut output_string = String::new();
		for digit in output {
			let chars = HashSet::<char>::from_iter(digit.chars());
			for i in 0u8..10 {
				if chars == patterns[i as usize] {
					output_string.push((i + b'0') as char);
					break;
				}
			}
		}

		total_output += output_string.parse::<u32>().unwrap();
	}

	total_output
}

#[test]
fn test_day_08() {
	assert_eq!(day_08("day_08_test.txt"), 26);
	assert_eq!(day_08_part2("day_08_test.txt"), 61229);
}
