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
	let vec = file_to_vec::<String>(file);

	let size = vec[0].len();
	let mut counts = vec![0usize; size];
	for num in &vec {
		let bin = u32::from_str_radix(&num, 2).unwrap();
		(0..size).for_each(|i| counts[i] += ((bin >> (size - i - 1)) & 1) as usize);
	}

	let len = vec.len();
	let gamma = (0..size).fold(0, |acc, i| acc | ((counts[i] > (len / 2)) as u32) << (size - i - 1));

	// I am ONLY doing this to ensure compatibility with >8 bit numbers
	gamma * (!gamma & (1 << size) - 1)
}

#[allow(dead_code)]
pub fn day_03_part2(file: &str) -> u32 {
	let vec = file_to_vec::<String>(file);

	5
}

#[test]
fn test_day_03() {
	assert_eq!(day_03("day_03_test.txt"), 198);
	assert_eq!(day_03_part2("day_o3_test.txt"), 230);
}
