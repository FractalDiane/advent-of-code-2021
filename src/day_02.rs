use crate::useful::file_to_vec;

// On the SECOND day of Rustmas, Ferris gave to me
// Two closure pipes ||

#[allow(dead_code)]
pub fn day_02() {
	let vec = file_to_vec::<String>("day_02.txt");

	let mut hpos = 0;
	let mut depth = 0;
	let mut aim = 0;

	for command in vec {
		let words: Vec<&str> = command.split_whitespace().collect();
		let amount = words[1].parse::<i32>().unwrap();
		match words[0] {
			"forward" => {
				hpos += amount;
				depth += aim * amount;
			},
			"down" => {
				aim += amount;
			},
			_ => {
				aim -= amount;
			},
		}
	}

	println!("{}", hpos * depth);
}
