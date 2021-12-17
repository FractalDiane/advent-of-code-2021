// Advent of Code 2021
// Day 17 - Trick Shot

// On the SEVENTEENTH day of Rustmas, Ferris gave to me
// Seventeen public functions

use crate::useful::file_to_vec;
use std::cmp::max;

fn fire_probe(mut velocity_x: i32, mut velocity_y: i32, target: (i32, i32, i32, i32)) -> (bool, i32) {
	let mut x = 0;
	let mut y = 0;
	let mut hit = false;
	let mut max_y = i32::MIN;
	for _ in 0..1000 {
		x += velocity_x;
		y += velocity_y;
		
		if !hit && x >= target.0 && x <= target.1 && y >= target.2 && y <= target.3 {
			hit = true;
		}

		if y > max_y {
			max_y = y;
		}

		velocity_x -= velocity_x.signum();
		velocity_y -= 1;
	}

	(hit, max_y)
}

#[allow(dead_code)]
pub fn day_17(file: &str, part2: bool) -> i32 {
	let input = &file_to_vec::<String>(file)[0];
	let split = input[15..].split(", y=").collect::<Vec<&str>>();
	let mut tx = split[0].split(".."); let mut ty = split[1].split("..");
	let tx_lo = tx.next().unwrap().parse::<i32>().unwrap();
	let tx_hi = tx.next().unwrap().parse::<i32>().unwrap();
	let ty_lo = ty.next().unwrap().parse::<i32>().unwrap();
	let ty_hi = ty.next().unwrap().parse::<i32>().unwrap();
	let target = (tx_lo, tx_hi, ty_lo, ty_hi);

	let bound_x = max(tx_lo.abs(), tx_hi.abs());
	let bound_y = max(ty_lo.abs(), ty_hi.abs());
	let mut max_y = i32::MIN;
	let mut hit_count = 0;
	for vx in -bound_x..=bound_x {
		for vy in -bound_y..=bound_y {
			let result = fire_probe(vx, vy, target);
			if result.0 {
				hit_count += 1;
				if result.1 > max_y {
					max_y = result.1;
				}
			}
		}
	}

	if !part2 {
		max_y
	} else {
		hit_count
	}
}

#[test]
fn test_day_17() {
	assert_eq!(day_17("day_17_test.txt", false), 45);
	assert_eq!(day_17("day_17_test.txt", true), 112);
}
