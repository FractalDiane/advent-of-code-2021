// Advent of Code 2021
// Day 18 - Snailfish

// On the EIGHTEENTH day of Rustmas, Ferris gave to me
// Eighteen enum'rations

use crate::useful::file_to_vec;

#[derive(Clone)]
struct FlattenedDigit {
	value: i32,
	depth: u32,
}

fn parse_pair(pair: &str) -> Vec<FlattenedDigit> {
	let mut result = vec![];
	let contents = &pair[1..pair.len() - 1];
	let mut depth = 0;
	for chr in contents.chars() {
		match chr {
			'[' => {
				depth += 1;
			},
			']' => {
				depth -= 1;
			},
			_ => {
				let c8 = chr as u8;
				if c8 >= b'0' && c8 <= b'9' {
					result.push(FlattenedDigit{value: (c8 - b'0') as i32, depth})
				}
			}
		}
	}

	result
}

fn add_nums(left: Vec<FlattenedDigit>, right: Vec<FlattenedDigit>) -> Vec<FlattenedDigit> {
	let mut result = vec![];
	result.extend(left);
	result.extend(right);

	for n in &mut result {
		n.depth += 1;
	}

	result
}

fn magnitude(num: &Vec<FlattenedDigit>) -> i32 {
	let mut result = num.clone();
	let mut depth = num.iter().map(|n| n.depth).max().unwrap();
	while depth > 0 {
		let mut i = 0;
		while i < result.len() - 1 {
			if result[i].depth == depth {
				let mag = result[i].value * 3 + result[i + 1].value * 2;
				let new_num = FlattenedDigit{value: mag, depth: depth - 1};
				result.remove(i);
				result.remove(i);
				result.insert(i, new_num);
			}

			i += 1;
		}

		depth -= 1;
	}

	result[0].value * 3 + result[1].value * 2
}

fn is_reduced(num: &Vec<FlattenedDigit>) -> bool {
	for digit in num {
		if digit.depth >= 4 || digit.value >= 10 {
			return false;
		}
	}

	true
}

fn reduce(num: &Vec<FlattenedDigit>) -> Vec<FlattenedDigit> {
	let mut result = num.clone();
	'top: while !is_reduced(&result) {
		let reference = result.clone();
		for i in 0..reference.len() {
			let digit = &reference[i];
			if digit.depth >= 4 {
				let left = result[i].value;
				let right = result[i + 1].value;
				result.remove(i);
				result.remove(i);
				result.insert(i, FlattenedDigit{value: 0, depth: digit.depth - 1});
				if i > 0 {
					result[i - 1].value += left;
				}

				if i < result.len() - 1 {
					result[i + 1].value += right;
				}

				continue 'top;
			}
		}

		for i in 0..reference.len() {
			let digit = &reference[i];
			if digit.value >= 10 {
				let left = digit.value / 2;
				let right = left + 1 * (digit.value % 2 != 0) as i32;
				result.remove(i);
				result.insert(i, FlattenedDigit{value: right, depth: digit.depth + 1});
				result.insert(i, FlattenedDigit{value: left, depth: digit.depth + 1});
				continue 'top;
			}
		}
	}

	result
}

#[allow(dead_code)]
pub fn day_18(file: &str, largest: bool) -> i32 {
	let input = file_to_vec::<String>(file);
	let mut numbers = Vec::<Vec<FlattenedDigit>>::new();
	for line in &input {
		let number = parse_pair(line);
		let reduced = reduce(&number);
		numbers.push(reduced);
	}

	if !largest {
		while numbers.len() > 1 {
			let left = numbers.remove(0);
			let right = numbers.remove(0);
			let result = add_nums(left, right);
			let reduced = reduce(&result);
			numbers.insert(0, reduced);
		}

		magnitude(&numbers[0])
	} else {
		let mut max = 0;
		for i in 0..numbers.len() {
			for j in 0..numbers.len() {
				let result = add_nums(numbers[i].clone(), numbers[j].clone());
				let reduced = reduce(&result);
				let mag = magnitude(&reduced);
				if mag > max {
					max = mag;
				}

			}
		}

		max
	}
}

#[test]
fn test_day_18() {
	assert_eq!(day_18("day_18_test.txt", false), 4140);
	assert_eq!(day_18("day_18_test.txt", true), 3993);
}
