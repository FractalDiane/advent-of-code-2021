use crate::useful::file_to_vec;

// On the FIRST day of Rustmas, Ferris gave to me
// A lack of a segfault I'd have had in C

#[allow(dead_code)]
pub fn day_01() {
	let vec = file_to_vec::<u32>("day_01.txt");
	
	let mut count = 0;
	let mut last_sum = 0u32;
	for i in 0..vec.len() - 2 {
		let sum = vec[i] + vec[i + 1] + vec[i + 2];
		if i > 0 && sum > last_sum {
			count += 1;
		}

		last_sum = sum;
	}

	println!("{}", count);
}
