// Advent of Code 2021
// Day 15 - Chiton

// On the FIFTEENTH day of Rustmas, Ferris gave to me
// Fifteen random numbers

use crate::useful::file_to_vec;
use std::collections::BTreeSet;

fn adjacent_indices(index: usize, row_size: usize) -> Vec<usize> {
	let mut ret = vec![index.wrapping_sub(row_size), index.saturating_add(row_size)];
	if index % row_size != 0 {
		ret.push(index.wrapping_sub(1));
	}

	if index % row_size != row_size - 1 {
		ret.push(index.saturating_add(1));
	}
	
	ret
}

#[allow(dead_code)]
pub fn day_15(file: &str, full: bool) -> u32 {
	let input = file_to_vec::<String>(file);
	let mut width = input[0].len();
	let mut graph = input.into_iter().fold(vec![], |mut vec, line| {
		vec.extend(line.chars().map(|n| n as u8 - b'0'));
		vec
	});

	if full {
		let mut new_graph = Vec::<u8>::new();
		for v in 0..5 {
			for i in 0..width {
				for j in 0..5 {
					for k in 0..width {
						let val = graph[width * i + k] + j + v;
						new_graph.push(((val - 1) % 9) + 1);
					}
				}
			}
		}

		graph = new_graph;
		width *= 5;
	}

	// This code is absolute garbage and takes 10 minutes on part 2 even in release mode
	// I don't care
	// Fuck pathfinding problems
	// I'll fix it later

	let mut distance = vec![u32::MAX; graph.len()];
	let mut previous = vec![usize::MAX; graph.len()];
	let mut frontier = (0..graph.len()).fold(BTreeSet::new(), |mut set, i| {
		set.insert(i);
		set
	});

	distance[0] = 0;

	while !frontier.is_empty() {
		let mut min = u32::MAX;
		let mut min_index = usize::MAX;
		for i in 0..distance.len() {
			if distance[i] < min && frontier.contains(&i) {
				min = distance[i];
				min_index = i;
			}
		}

		frontier.remove(&min_index);

		if min_index == graph.len() - 1 {
			let mut cost = 0u32;
			let mut current = graph.len() - 1;
			while current != usize::MAX {
				cost += graph[current] as u32;
				current = previous[current];
			}

			return cost - graph[0] as u32;
		}
		
		for neighbor in adjacent_indices(min_index, width) {
			if neighbor < graph.len() && frontier.contains(&neighbor) {
				let temp = distance[min_index].saturating_add(graph[neighbor] as u32);
				if temp < distance[neighbor] {
					distance[neighbor] = temp;
					previous[neighbor] = min_index;
				}
			}
		}
	}

	u32::MAX
}

#[test]
fn test_day_15() {
	//assert_eq!(day_15("day_15_test.txt", false), 40);
	//assert_eq!(day_15("day_15_test.txt", true), 315);
}
