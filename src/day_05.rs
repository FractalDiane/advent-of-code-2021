// Advent of Code 2021
// Day 5 - Hydrothermal Venture

// On the FIFTH day of Rustmas, Ferris gave to me
// FIIIIIIIIIIIIIIIIVE FOLDED STRINGS
//
// println!("{0}\n{0}\n{0}\n{0}\n{0}", vec!['H','A','P','P','Y',' ','H','O','L','I','D','A','Y','S'].into_iter().fold(String::new(), |mut s, c| { s.push(c); s }));

use crate::useful::file_to_vec;
use std::collections::HashMap;
use std::cmp::max;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Point {
	x: i32,
	y: i32,
}

#[allow(dead_code)]
pub fn day_05(file: &str, check_diagonals: bool) -> u32 {
	let input = file_to_vec::<String>(file);
	let mut point_counts = HashMap::<Point, u32>::new();

	for line in input {
		let pts = line.split(" -> ").fold(Vec::<i32>::new(), |mut vec, n| {
			n.split(',').for_each(|num| vec.push(num.parse::<i32>().unwrap()));
			vec
		});

		let x1 = pts[0]; let y1 = pts[1];
		let x2 = pts[2]; let y2 = pts[3];

		if check_diagonals || x1 == x2 || y1 == y2 {
			let diff = max((x2 - x1).abs(), (y2 - y1).abs());
			let dx = (x1 < x2) as i8 * 2 - 1;
			let dy = (y1 < y2) as i8 * 2 - 1;
			let cx = (x1 != x2) as i8;
			let cy = (y1 != y2) as i8;

			let mut point = Point{x: x1, y: y1};
			for _ in 0..=diff {
				*point_counts.entry(point.clone()).or_insert(0) += 1;
				point.x += (dx * cx) as i32;
				point.y += (dy * cy) as i32;
			}
		}
	}

	point_counts.into_values().filter(|v| v > &1).count() as u32
}

#[test]
fn test_day_05() {
	assert_eq!(day_05("day_05_test.txt", false), 5);
	assert_eq!(day_05("day_05_test.txt", true), 12);
}
