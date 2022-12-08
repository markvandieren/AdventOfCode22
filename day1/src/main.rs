#![warn(clippy::all)]
#![warn(clippy::pedantic)]

const INPUT: &str = include_str!("../data/input.txt");

fn main() {
	let mut elves = Vec::new();
	let mut elf = 0;
	for line in INPUT.lines().map(str::trim) {
		if line.is_empty() {
			elves.push(elf);
			elf = 0;
		} else {
			elf += line.parse::<i32>().unwrap();
		}
	}

	if elf != 0 {
		elves.push(elf);
	}

	elves.sort_unstable();
	elves.reverse();

	println!("Top 1: {}", elves.first().unwrap());
	println!("Top 3: {}", elves.iter().take(3).sum::<i32>());
}
