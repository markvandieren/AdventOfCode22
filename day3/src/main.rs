#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
#![allow(dead_code)]

mod task1;

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
	let mut group = Vec::new();

	let mut sum = 0;
	for rucksack_contents in input.lines() {
		group.push(rucksack_contents);
		if group.len() == 3 {
			sum += priority(find_badge(&group));
			group.clear();
		}
	}

	sum
}

fn rucksack_contains(rucksack: &str, item: char) -> bool {
	rucksack.chars().any(|rucksack_item| rucksack_item == item)
}

fn find_badge(group: &[&str]) -> char {
	let (first_rucksack, other_rucksacks) = group.split_first().unwrap();
	first_rucksack
		.chars()
		.filter(|first_rucksack_item| {
			other_rucksacks
				.iter()
				.all(|other_rucksack| rucksack_contains(other_rucksack, *first_rucksack_item))
		})
		.take(1)
		.next()
		.unwrap()
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
		let group = vec![
			"vJrwpWtwJgWrhcsFMMfFFhFp",
			"jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
			"PmmdzqPrVvPwwTWBwg",
		];
		assert_eq!(find_badge(&group), 'r');
	}

	{
		let group = vec![
			"wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
			"ttgJtRGJQctTZtZT",
			"CrZsJsPPZsGzwwsLwLmpwMDw",
		];
		assert_eq!(find_badge(&group), 'Z');
	}

	assert_eq!(run(SAMPLE), 70);
}
