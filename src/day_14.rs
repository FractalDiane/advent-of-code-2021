// Advent of Code
// Day 14 - Extended Polymerization

// On the FOURTEENTH day of Rustmas, Ferris gave to me
// Fourteen Vectors growing

use crate::useful::file_to_vec;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn day_14(file: &str, steps: u32) -> u64 {
	let input = file_to_vec::<String>(file);
	
	let template = input[0].chars().collect::<Vec<char>>();
	let rules = input[2..].into_iter().fold(HashMap::new(), |mut map, r| {
		let mut split = r.split(" -> ");
		map.insert(split.next().unwrap(), split.next().unwrap().chars().next().unwrap());
		map
	});

	let mut counts = template.iter().fold(HashMap::new(), |mut map, c| {
		*map.entry(*c).or_insert(0u64) += 1;
		map
	});

	let mut pairs = HashMap::<String, u64>::new();
	for i in 0..template.len() - 1 {
		let pair: String = template[i..=i + 1].into_iter().collect();
		*pairs.entry(pair).or_insert(0) += 1;
	}

	for _i in 0..steps {
		let mut new_pairs = HashMap::<String, u64>::new();
		for pair in pairs.keys() {
			match rules.get(pair.as_str()) {
				Some(r) => {
					let mut chars = pair.chars();
					let left = chars.next().unwrap();
					let right = chars.next().unwrap();

					let current_count = pairs[pair];
					let new_left: String = [left, *r].iter().collect();
					let new_right: String = [*r, right].iter().collect();

					*new_pairs.entry(new_left).or_default() += current_count;
					*new_pairs.entry(new_right).or_default() += current_count;
					
					*counts.entry(*r).or_default() += current_count;
				},
				None => {},
			}
		}

		pairs = new_pairs;
	}

	counts.values().max().unwrap() - counts.values().min().unwrap()
}

#[test]
fn test_day_14() {
	assert_eq!(day_14("day_14_test.txt", 10), 1588);
	assert_eq!(day_14("day_14_test.txt", 40), 2188189693529);
}
