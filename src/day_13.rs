// Advent of Code 2021
// Day 13 - Transparent Origami

// On the THIRTEENTH day of Rustmas, Ferris gave to me
// Thirteen unwrapped Options

use crate::useful::file_to_vec;

struct Dot {
	x: u32,
	y: u32,
}

struct Fold {
	y: bool,
	coord: u32,
}

fn print_paper(paper: &Vec<bool>, width: u32) {
	let mut string = String::new();
	for i in 0..paper.len() as u32 {
		string.push(if paper[i as usize] { '#' } else { '.' });
		if i % width == width - 1 {
			string.push('\n');
		}
	}

	println!("{}", string);
}

#[allow(dead_code)]
pub fn day_13(file: &str, all_folds: bool) -> u32 {
	let input = file_to_vec::<String>(file);

	// Oh BOY is this one annoying to set up
	let mut dots = Vec::<Dot>::new();
	let mut folds = Vec::<Fold>::new();
	let mut max_x = 0;
	let mut max_y = 0;
	for line in input {
		if line.starts_with("fold") {
			let instr = line.split_whitespace().nth(2).unwrap();
			folds.push(Fold{
				y: instr.chars().next().unwrap() == 'y',
				coord: instr[2..].parse::<u32>().unwrap(),
			});
		} else if !line.is_empty() {
			let mut split = line.split(',');
			let x = split.next().unwrap().parse::<u32>().unwrap();
			let y = split.next().unwrap().parse::<u32>().unwrap();
			dots.push(Dot{x, y});
			if x > max_x {
				max_x = x;
			}

			if y > max_y {
				max_y = y;
			}
		}
	}

	let mut x_width = max_x + 1;
	let y_height = max_y + 1;
	let mut paper = (0..x_width * y_height).fold(vec![], |mut vec, _| { vec.push(false); vec});
	for dot in &dots {
		paper[(dot.y * x_width + dot.x) as usize] = true;
	}

	// ...And with that we're finally set up
	for fold in &folds[0..=(0 + (folds.len() - 1) * all_folds as usize)] {
		for i in 0..paper.len() as u32 {
			if paper[i as usize] {
				let x = i % x_width;
				let y = i / x_width;
				if fold.y && i >= x_width * fold.coord {
					let new_y = max_y - y;
					paper[(new_y * x_width + x) as usize] = true;
				} else if !fold.y && i % x_width > fold.coord {
					let new_x = max_x - x;
					paper[(y * x_width + new_x) as usize] = true;
					paper[i as usize] = false;
				}
			}
		}

		if fold.y {
			let new_height = max_y - fold.coord;
			paper.resize((new_height * x_width) as usize, false);

			max_y = new_height - 1;
		} else {
			let new_width = max_x - fold.coord;
			let mut new_vec = Vec::<bool>::new();
			for i in 0..paper.len() as u32 {
				if i % x_width < new_width {
					new_vec.push(paper[i as usize]);
				}
			}

			paper = new_vec;
			x_width = new_width;
			max_x = new_width - 1;
		}
	}

	if all_folds {
		// Huge thanks to RamieShreim for helping me actually read the letters
		print_paper(&paper, x_width);
	}

	paper.iter().filter(|b| **b).count() as u32
}

#[test]
fn test_day_13() {
	assert_eq!(day_13("day_13_test.txt", false), 17);
}
