// Advent of Code 2021
// Day 5 -

// On the FIFTH day of Rustmas, Ferris gave to me
// FIIIIIIIIIIIIIIIIVE FOLDED STRINGS
//
// println!("{0}\n{0}\n{0}\n{0}\n{0}", vec!['H','A','P','P','Y',' ','H','O','L','I','D','A','Y','S'].into_iter().fold(String::new(), |mut s, c| { s.push(c); s }));

use crate::useful::file_to_vec;
use std::collections::HashMap;
use std::cmp::{min, max};

#[derive(PartialEq, Eq, Hash)]
struct Point {
	pub x: u32,
	pub y: u32,
}

#[allow(dead_code)]
pub fn day_05(file: &str) -> u32 {
	let input = file_to_vec::<String>(file);
	let mut point_counts = HashMap::<Point, u32>::new();

	for line in input {
		let pts = line.split(" -> ").fold(Vec::<u32>::new(), |mut vec, n| {
			n.split(',').for_each(|num| vec.push(num.parse::<u32>().unwrap()));
			vec
		});
		
		let x1 = min(pts[0], pts[2]); let y1 = min(pts[1], pts[3]);
		let x2 = max(pts[0], pts[2]); let y2 = max(pts[1], pts[3]);

		if x1 == x2 || y1 == y2 {
			let y_diff = y1 != y2;
			for pt in if y_diff { y1..=y2 } else { x1..=x2 } {
				let x = if y_diff { x1 } else { pt };
				let y = if y_diff { pt } else { y1 };
				*point_counts.entry(Point{x, y}).or_insert(0) += 1;
			}
		}
	}

	point_counts.into_values().filter(|v| v > &1).count() as u32
}

#[test]
fn test_day_05() {
	assert_eq!(day_05("day_05_test.txt"), 5);
}
