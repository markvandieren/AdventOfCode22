#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
#![allow(dead_code)]

const INPUT: &str = include_str!("../data/input.txt");

fn main() {
	assert_eq!(run("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7);
	assert_eq!(run("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
	assert_eq!(run("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
	assert_eq!(run("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
	assert_eq!(run("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
	println!("Input: {}", run(INPUT, 4));

	assert_eq!(run("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
	assert_eq!(run("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
	assert_eq!(run("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
	assert_eq!(run("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
	assert_eq!(run("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
	println!("Input: {}", run(INPUT, 14));
}

fn run(s: &str, num_chars: usize) -> usize {
	let mut buffer = Vec::new();
	for (index, c) in s.chars().enumerate() {
		buffer.push(c);
		if buffer.len() > num_chars {
			buffer.remove(0);
		}
		let mut buffer2 = buffer.clone();
		buffer2.sort_unstable();
		buffer2.dedup();
		if buffer2.len() == num_chars {
			return index + 1;
		}
	}

	panic!()
}
