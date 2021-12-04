// Advent of Code 2021
// Day 3 - Binary Diagnostic

// On the THIRD day of Rustmas, Ferris gave to me
// Three macros
//
// println!("Deck the halls");
// eprintln!("We appear to have no halls to deck");
// panic!();

use crate::useful::file_to_vec;

#[allow(dead_code)]
pub fn day_03(file: &str) -> u32 {
	let input = file_to_vec::<String>(file);

	let size = input[0].len();
	let mut counts = vec![0usize; size];
	for num in &input {
		let bin = u32::from_str_radix(&num, 2).unwrap();
		(0..size).for_each(|i| counts[i] += ((bin >> (size - i - 1)) & 1) as usize);
	}

	let len = input.len();
	let gamma = (0..size).fold(0, |acc, i| acc | ((counts[i] > (len / 2)) as u32) << (size - i - 1));

	// I am ONLY using u32s and by extension doing this to ensure compatibility with >8 bit numbers
	gamma * (!gamma & (1 << size) - 1)
}

fn trim_rating(mut input: Vec<String>, size: usize, mut counts: Vec<usize>, co2: bool) -> u32 {
	let mut i = size - 1;
	let mut len = input.len();
	while input.len() > 1 {
		let mut ones_removed = vec![0usize; size];
		let old_len = input.len();
		input.retain(|num| {
			let bin = u32::from_str_radix(&num, 2).unwrap();
			let bit = (bin >> i) & 1;

			let check = !co2 as u32;
			let retain = bit == if len % 2 == 0 && counts[size - i - 1] == len / 2 { check } else if counts[size - i - 1] > len / 2 { check } else { 1 - check };
			if !retain {
				(0..size).for_each(|i| ones_removed[i] += ((bin >> (size - i - 1)) & 1) as usize);
			}

			retain
		});

		(0..size).for_each(|i| counts[i] -= ones_removed[i]);
		len -= old_len - input.len();
		i = i.wrapping_sub(1);
	}

	u32::from_str_radix(&input[0], 2).unwrap()
}

#[allow(dead_code)]
pub fn day_03_part2(file: &str) -> u32 {
	let input = file_to_vec::<String>(file);

	let size = input[0].len();
	let mut counts = vec![0usize; size];
	for num in &input {
		let bin = u32::from_str_radix(&num, 2).unwrap();
		(0..size).for_each(|i| counts[i] += ((bin >> (size - i - 1)) & 1) as usize);
	}

	let o2 = trim_rating(input.clone(), size, counts.clone(), false);
	let co2 = trim_rating(input, size, counts, true);

	o2 * co2
}

#[test]
fn test_day_03() {
	assert_eq!(day_03("day_03_test.txt"), 198);
	assert_eq!(day_03_part2("day_03_test.txt"), 230);
}
