use crate::useful::file_to_vec;

// Advent of Code 2021
// Day 16 - Packet Decoder

// On the SIXTEENTH day of Rustmas, Ferris gave to me
// Sixteen substring slices

fn hex_to_bin(hex: &str) -> String {
	let mut result = String::new();

	for chr in hex.chars() {
		let num = u8::from_str_radix(&chr.to_string(), 16).unwrap();
		for i in 0..4 {
			result.push(((num >> 3 - i & 1) + b'0') as char);
		}
	}

	result
}

fn parse_packet(bin: &str) -> (u64, usize, u64) {
	let mut len = 0;
	let mut version = u64::from_str_radix(&bin[0..3], 2).unwrap();
	let type_id = u8::from_str_radix(&bin[3..6], 2).unwrap();
	len += 6;

	if type_id == 4 {
		let mut literal = 0u64;
		let mut i = 0;
		loop {
			let current = u64::from_str_radix(&bin[6 + 5 * i..11 + 5 * i], 2).unwrap();
			literal |= current & 0b1111;
			len += 5;

			if current & 0b10000 == 0 {
				break;
			}

			literal <<= 4;
			i += 1;
		}

		return (literal, len, version)
	} else {
		let length_type = u8::from_str_radix(&bin[6..=6], 2).unwrap() == 1;
		let mut packet_results = Vec::<u64>::new();
		len += 1;
		let mut sub_len = 0;
		if !length_type {
			let mut bit_count = u16::from_str_radix(&bin[7..22], 2).unwrap() as i16;
			len += 15;
			while bit_count > 0 {
				let subpacket = parse_packet(&bin[22 + sub_len..]);
				let this_len = subpacket.1;
				sub_len += this_len;
				bit_count -= this_len as i16;
				version += subpacket.2;
				packet_results.push(subpacket.0);
			}
		} else {
			let packet_count = u16::from_str_radix(&bin[7..18], 2).unwrap();
			len += 11;
			for _ in 0..packet_count {
				let subpacket = parse_packet(&bin[18 + sub_len..]);
				let this_len = subpacket.1;
				sub_len += this_len;
				version += subpacket.2;
				packet_results.push(subpacket.0);
			}
		}

		let result = match type_id {
			0 => packet_results.iter().sum(),
			1 => packet_results.iter().product(),
			2 => *packet_results.iter().min().unwrap(),
			3 => *packet_results.iter().max().unwrap(),
			5 => (packet_results[0] > packet_results[1]) as u64,
			6 => (packet_results[0] < packet_results[1]) as u64,
			_ => (packet_results[0] == packet_results[1]) as u64,
		};

		(result, len + sub_len, version)
	}
}

#[allow(dead_code)]
pub fn day_16(file: &str, do_operations: bool) -> u64 {
	let input = file_to_vec::<String>(file);
	let packet = &input[0];

	if !do_operations {
		parse_packet(&hex_to_bin(packet)).2
	} else {
		parse_packet(&hex_to_bin(packet)).0
	}
}

#[test]
fn test_day_16() {
	assert_eq!(hex_to_bin("ABC"), "101010111100");
	assert_eq!(parse_packet(&hex_to_bin("D2FE28")).0, 2021);
	
	assert_eq!(parse_packet(&hex_to_bin("8A004A801A8002F478")).2, 16);
	assert_eq!(parse_packet(&hex_to_bin("C0015000016115A2E0802F182340")).2, 23);
	assert_eq!(parse_packet(&hex_to_bin("A0016C880162017C3686B18A3D4780")).2, 31);

	assert_eq!(parse_packet(&hex_to_bin("C200B40A82")).0, 3);
	assert_eq!(parse_packet(&hex_to_bin("04005AC33890")).0, 54);
	assert_eq!(parse_packet(&hex_to_bin("880086C3E88112")).0, 7);
	assert_eq!(parse_packet(&hex_to_bin("CE00C43D881120")).0, 9);
	assert_eq!(parse_packet(&hex_to_bin("D8005AC2A8F0")).0, 1);
	assert_eq!(parse_packet(&hex_to_bin("F600BC2D8F")).0, 0);
	assert_eq!(parse_packet(&hex_to_bin("9C005AC2F8F0")).0, 0);
	assert_eq!(parse_packet(&hex_to_bin("9C0141080250320F1802104A08")).0, 1);
}
