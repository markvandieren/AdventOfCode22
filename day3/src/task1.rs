const INPUT: &str = include_str!("../data/input.txt");
const SAMPLE: &str = include_str!("../data/sample.txt");

fn main() {
	println!("Sample: ");
	println!("Sample sum: {}", run(SAMPLE));
	println!("Input: ");
	println!("Input sum: {}", run(INPUT));
}

const COLUMN_WIDTH: usize = 25;
fn run(input: &str) -> i32 {
	input
		.lines()
		.map(|rucksack_contents| {
			let (first, second) = split(rucksack_contents);
			let common = common(first, second);
			let priority = priority(common);
			println!("{first:>COLUMN_WIDTH$} | {second:<COLUMN_WIDTH$} -> {common} = {priority}");
			priority
		})
		.sum()
}

fn split(rucksack_contents: &str) -> (&str, &str) {
	rucksack_contents.split_at(rucksack_contents.len() / 2)
}

fn common(first_compartment: &str, second_compartment: &str) -> char {
	for item in first_compartment.chars() {
		if second_compartment.contains(item) {
			return item;
		}
	}
	panic!("No common item");
}

fn priority(item: char) -> i32 {
	if item.is_lowercase() {
		item as i32 - 'a' as i32 + 1
	} else {
		item as i32 - 'A' as i32 + 27
	}
}

#[test]
fn test() {
	{
		let (a, b) = split("vJrwpWtwJgWrhcsFMMfFFhFp");
		assert_eq!((a, b), ("vJrwpWtwJgWr", "hcsFMMfFFhFp"));
		assert_eq!(common(a, b), 'p');
		assert_eq!(priority('p'), 16);
	}

	{
		let (a, b) = split("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL");
		assert_eq!((a, b), ("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL"));
		assert_eq!(common(a, b), 'L');
		assert_eq!(priority('L'), 38);
	}

	{
		let (a, b) = split("PmmdzqPrVvPwwTWBwg");
		assert_eq!((a, b), ("PmmdzqPrV", "vPwwTWBwg"));
		assert_eq!(common(a, b), 'P');
		assert_eq!(priority('P'), 42);
	}

	{
		let (a, b) = split("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn");
		assert_eq!(common(a, b), 'v');
		assert_eq!(priority('v'), 22);
	}

	{
		let (a, b) = split("ttgJtRGJQctTZtZT");
		assert_eq!(common(a, b), 't');
		assert_eq!(priority('t'), 20);
	}

	{
		let (a, b) = split("CrZsJsPPZsGzwwsLwLmpwMDw");
		assert_eq!(common(a, b), 's');
		assert_eq!(priority('s'), 19);
	}

	assert_eq!(run(SAMPLE), 157);
}
