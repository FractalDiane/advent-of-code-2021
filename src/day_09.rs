// Advent of code 2021
// Day 9 - Smoke Basin

// On the NINTH day of Rustmas, Ferris gave to me
// Nine Boxes pointing

use crate::useful::file_to_vec;
use std::collections::HashSet;

fn adjacent_indices(index: i32, row_size: i32) -> Vec<i32> {
	vec![
		if index % row_size != 0 { index - 1 } else { -1 },
		if index % row_size != row_size - 1 { index + 1 } else { -1 },
		index - row_size,
		index + row_size,
	]
}

fn basin_size(points: &Vec<u8>, visited: &mut HashSet<i32>, row_size: i32, index: i32, lowest_value: u8) -> u32 {
	visited.insert(index);
	let indices = adjacent_indices(index, row_size);
	indices.into_iter().fold(0, |acc, i| {
		let value = *points.get(i as usize).unwrap_or(&10);
		if value < 9 && value > lowest_value && !visited.contains(&i) {
			acc + basin_size(points, visited, row_size, i, value)
		} else {
			acc + 0
		}
	}) + 1
}

#[allow(dead_code)]
pub fn day_09(file: &str, basins: bool) -> u32 {
	let input = file_to_vec::<String>(file);

	let row_size = input[0].len() as i32;
	let mut points = Vec::<u8>::new();
	for line in input {
		line.chars().for_each(|n| points.push(n as u8 - b'0'));
	}

	let mut low_points = Vec::<(u8, i32)>::new();
	for i in 0..points.len() as i32 {
		let indices = adjacent_indices(i, row_size);
		let low = indices.into_iter().all(|ind| {
			points[i as usize] < *points.get(ind as usize).unwrap_or(&10)
		});
		
		if low {
			low_points.push((points[i as usize], i));
		}
	}

	if !basins {
		low_points.into_iter().fold(0, |acc, (p, _i)| acc + p as u32 + 1)
	} else {
		let mut visited_points = HashSet::<i32>::new();
		let mut sizes = low_points.into_iter().fold(Vec::new(), |mut vec, (p, i)| {
			let size = basin_size(&points, &mut visited_points, row_size, i, p);
			vec.push(size);
			vec
		});

		sizes.sort_unstable_by(|l, r| r.cmp(l));
		sizes[0..3].into_iter().fold(1, |acc, n| acc * n)
	}
}

#[test]
fn test_day_09() {
	assert_eq!(day_09("day_09_test.txt", false), 15);
	assert_eq!(day_09("day_09_test.txt", true), 1134);
}
