// Advent of Code 2021
// Day 10 - Syntax Scoring

// On the TENTH day of Rustmas, Ferris gave to me
// Ten crates a-scoping

use crate::useful::file_to_vec;

#[allow(dead_code)]
pub fn day_10(file: &str, completion_mode: bool) -> u64 {
	let input = file_to_vec::<String>(file);
	let opens = ['(', '[', '{', '<'];
	
	let mut stack = Vec::<char>::new();
	let mut score = 0u64;
	let mut incomplete_scores = Vec::<u64>::new();

	for line in input {
		let mut corrupted = false;
		for c in line.chars() {
			if opens.contains(&c) {
				stack.push(c);
			} else {
				let opposite = (c as u8 - 1 - (c != ')') as u8) as char;
				if *stack.last().unwrap_or(&'x') == opposite {
					stack.pop();
				} else {
					if !completion_mode {
						score += match c {
							')' => 3,
							']' => 57,
							'}' => 1197,
							_ => 25137,
						};
					}
					
					corrupted = true;
					break;
				}
			}
		}

		if !stack.is_empty() {
			if completion_mode && !corrupted {
				let mut new_score = 0;
				while !stack.is_empty() {
					let next = stack.pop().unwrap();
					new_score *= 5;
					new_score += match next {
						'(' => 1,
						'[' => 2,
						'{' => 3,
						_ => 4,
					};
				}

				incomplete_scores.push(new_score);
			}

			stack.clear();
		}
	}

	if !completion_mode {
		score
	} else {
		incomplete_scores.sort_unstable();
		incomplete_scores[incomplete_scores.len() / 2]
	}
}

#[test]
fn test_day_10() {
	assert_eq!(day_10("day_10_test.txt", false), 26397);
	assert_eq!(day_10("day_10_test.txt", true), 288957);
}
