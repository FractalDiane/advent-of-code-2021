// Advent of Code 2021
// Day 11 - Dumbo Octopus

// On the ELEVENTH day of Rustmas, Ferris gave to me
// Eleven linters linting

use crate::useful::file_to_vec;
use std::collections::HashSet;

fn adjacent_indices(index: usize, row_size: usize) -> Vec<usize> {
	let mut ret = vec![index.wrapping_sub(row_size), index + row_size];
	if index % row_size != 0 {
		ret.push(index.wrapping_sub(1));
		ret.push(index.wrapping_sub(row_size).wrapping_sub(1));
		ret.push(index + row_size - 1);
	}

	if index % row_size != row_size - 1 {
		ret.push(index + 1);
		ret.push(index.wrapping_sub(row_size) + 1);
		ret.push(index + row_size + 1);
	}
	
	ret
}

fn flash(index: usize, row_size: usize, octopuses: &mut Vec<u8>, flashed: &mut HashSet<usize>, check_indices: &mut Vec<usize>) {
	for ind in adjacent_indices(index, row_size) {
		match octopuses.get_mut(ind) {
			Some(oct) => {
				*oct += 1;
				if *oct > 9 && !flashed.contains(&ind) {
					check_indices.push(ind);
					flashed.insert(ind);
				}
			},
			None => {},
		}
	}
}

#[allow(dead_code)]
pub fn day_11(file: &str, iterations: u32, first_total_flash: bool) -> u32 {
	let input = file_to_vec::<String>(file);
	let row_size = input[0].len();
	let mut octopuses = Vec::<u8>::new();
	for line in input {
		line.chars().for_each(|c| {
			octopuses.push(c as u8 - b'0');
		});
	}

	let mut flash_count = 0u32;
	for iter in 0..iterations {
		let mut check_indices = Vec::<usize>::new();
		let mut flashed = HashSet::<usize>::new();
		
		octopuses.iter_mut().enumerate().for_each(|(i, o)| {
			*o += 1;
			if *o > 9 {
				check_indices.push(i);
				flashed.insert(i);
			}
		});

		while !check_indices.is_empty() {
			let inds = check_indices.clone();
			check_indices.clear();
			for ind in inds {
				flash(ind, row_size, &mut octopuses, &mut flashed, &mut check_indices);
			}
		}

		octopuses.iter_mut().filter(|o| **o > 9).for_each(|o| *o = 0);
		flash_count += flashed.len() as u32;

		if first_total_flash && flashed.len() == octopuses.len() {
			return iter + 1;
		}
	}
	
	flash_count
}

#[test]
fn test_day_11() {
	assert_eq!(day_11("day_11_test.txt", 100, false), 1656);
	assert_eq!(day_11("day_11_test.txt", 200, true), 195);
}
