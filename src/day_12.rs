// Advent of Code 2021
// Day 12 - Passage Pathing

// On the TWELFTH day of Rustmas, Ferris gave to me
// Twelve hashers hashing

use crate::useful::file_to_vec;
use std::collections::{HashSet, HashMap};

struct Cave {
	large: bool,
	adjacent: HashSet<String>,
}

fn traverse(cave: &String, cave_map: &HashMap<String, Cave>, visited: &mut HashMap<String, u8>, this_path: &mut Vec<String>, paths_taken: &mut HashSet<Vec<String>>, visit_twice: &String) {
	this_path.push(cave.into());
	if !&cave_map[cave].large {
		*visited.get_mut(cave.into()).unwrap() += 1;
	}

	if cave == "end" {
		paths_taken.insert(this_path.to_vec());
	} else {
		for c in &cave_map[cave].adjacent {
			if visited[c] < 1 + (c == visit_twice && c != "start" && c != "end") as u8 {
				traverse(c, cave_map, visited, this_path, paths_taken, visit_twice);
			}
		}
	}

	this_path.pop();
	let val = visited[cave];
	*visited.get_mut(cave).unwrap() = val.saturating_sub(1);
}

#[allow(dead_code)]
pub fn day_12(file: &str, visit_twice: bool) -> u32 {
	let input = file_to_vec::<String>(file);
	let mut caves = HashMap::<String, Cave>::new();
	for line in input {
		let split = line.split('-').collect::<Vec<&str>>();
		let c1 = split[0];
		let c2 = split[1];
		for i in 0..2 {
			if caves.contains_key(split[i].into()) {
				caves.get_mut(split[i]).unwrap().adjacent.insert(if i == 0 { c2.into() } else { c1.into() });
			} else {
				let mut adj = HashSet::<String>::new();
				adj.insert(if i == 0 { c2.into() } else { c1.into() });
				caves.insert(split[i].into(), Cave{
					large: split[i] == split[i].to_uppercase(),
					adjacent: adj,
				});
			}
		}
	}

	let mut visited = caves.iter().fold(HashMap::new(), |mut map, c| { map.insert(c.0.into(), 0); map });
	let mut paths_taken = HashSet::<Vec<String>>::new();
	
	if !visit_twice {
		traverse(&"start".into(), &caves, &mut visited, &mut vec![], &mut paths_taken, &String::new());
	} else {
		for cave in &caves {
			traverse(&"start".into(), &caves, &mut visited, &mut vec![], &mut paths_taken, &cave.0);
		}
	}

	paths_taken.len() as u32
}

#[test]
fn test_day_12() {
	assert_eq!(day_12("day_12_test.txt", false), 10);
	assert_eq!(day_12("day_12_test2.txt", false), 19);
	assert_eq!(day_12("day_12_test3.txt", false), 226);

	assert_eq!(day_12("day_12_test.txt", true), 36);
	assert_eq!(day_12("day_12_test2.txt", true), 103);
	assert_eq!(day_12("day_12_test3.txt", true), 3509);
}
