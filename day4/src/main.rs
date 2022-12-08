#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
#![allow(dead_code)]

const INPUT: &str = include_str!("../data/input.txt");
const SAMPLE: &str = include_str!("../data/sample.txt");

fn main() {
	println!("Sample contained: {}", task1_fully_contained(SAMPLE));
	println!("Input contained: {}", task1_fully_contained(INPUT));

	println!("Sample overlaps: {}", task2_overlaps(SAMPLE));
	println!("Input overlaps: {}", task2_overlaps(INPUT));
}

#[derive(Clone, Copy, Debug)]
struct Range(i32, i32);

impl Range {
	pub fn min(self) -> i32 {
		self.0
	}

	pub fn max(self) -> i32 {
		self.1
	}

	pub fn contains(self, other: Self) -> bool {
		other.min() >= self.min() && other.max() <= self.max()
	}

	pub fn overlaps(self, other: Self) -> bool {
		let b = !(self.max() < other.min() || self.min() > other.max());
		println!(
			"Overlap? [{}-{}] and [{} - {}] = {b}",
			self.0, self.1, other.0, other.1
		);
		b
	}
}

fn task1_fully_contained(input: &str) -> i32 {
	let mut count = 0;
	for line in input.lines() {
		let (a, b) = parse_line(line);

		if a.contains(b) || b.contains(a) {
			count += 1;
		}
	}

	count
}

fn task2_overlaps(input: &str) -> i32 {
	let mut count = 0;
	for line in input.lines() {
		let (a, b) = parse_line(line);

		if a.overlaps(b) {
			count += 1;
		}
	}

	count
}

fn parse_line(s: &str) -> (Range, Range) {
	let (a, b) = s.split_once(',').unwrap();
	(parse_range(a), parse_range(b))
}

fn parse_range(s: &str) -> Range {
	let (begin, end) = s.split_once('-').unwrap();
	Range(begin.parse::<i32>().unwrap(), end.parse::<i32>().unwrap())
}
