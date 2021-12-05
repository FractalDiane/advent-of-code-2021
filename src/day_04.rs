// Advent of Code 2021
// Day 4 - Giant Squid

// On the FOURTH day of Rustmas, Ferris gave to me
// Four test asserts
// 
// assert_eq!(day, 4);
// assert_eq!(month, 12);
// assert_eq!(year, 2021);
// assert!(rust_is_great);

use crate::useful::file_to_vec;
use std::collections::HashSet;

fn is_win(board: &Vec<u16>, called: &HashSet<u16>, last_called: u16) -> u32 {
	for i in 0..5 {
		let horizontal = |j| called.contains(&board[(i * 5 + j) as usize]);
		let vertical = |j| called.contains(&board[(i + 5 * j) as usize]);

		if (0..5).all(horizontal) || (0..5).all(vertical) {
			let sum = board.into_iter().filter(|num| !called.contains(num)).fold(0, |acc, n| acc + n);
			return sum as u32 * last_called as u32;
		}
	}

	0
}

#[allow(dead_code)]
pub fn day_04(file: &str, last_winner: bool) -> u32 {
	let input = file_to_vec::<String>(file);

	let calls = input[0].split(',').collect::<Vec<&str>>();

	let mut boards = Vec::<Vec::<u16>>::new();
	let board_count = (input.len() - 1) / 6;
	for i in 0..board_count {
		boards.push(Vec::new());
		for j in 0..5 {
			let row = &input[2 + 6 * i + j];
			row.split_whitespace().into_iter().for_each(|n| boards[i].push(n.parse::<u16>().unwrap()));
		}
	}

	let mut called = HashSet::<u16>::new();
	let mut boards_won = if last_winner { HashSet::<u16>::new() } else { HashSet::with_capacity(0) };

	for call in calls {
		let call_num = call.parse::<u16>().unwrap();
		called.insert(call_num);

		let mut i = 0;
		for board in &boards {
			if !last_winner || !boards_won.contains(&i) {
				let check = is_win(board, &called, call_num);
				if check != 0 {
					if !last_winner {
						return check;
					}
					else {
						boards_won.insert(i);
						if boards_won.len() == board_count {
							return check;
						}
					}
				}
			}
			
			i += 1;
		}
	}

	0
}

#[test]
fn day_04_test() {
	assert_eq!(day_04("day_04_test.txt", false), 4512);
	assert_eq!(day_04("day_04_test.txt", true), 1924);
}
