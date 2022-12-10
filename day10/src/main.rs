#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::wildcard_imports)]
#![allow(dead_code)]

mod task1;
mod task2;

const INPUT: &str = include_str!("../data/input.txt");
const SAMPLE: &str = include_str!("../data/sample.txt");

fn main() {
	println!("1. Sample: {}", task1::run(SAMPLE));
	println!("   Input: {}", task1::run(INPUT));

	println!("2. Sample");
	task2::run(SAMPLE);
	println!("   Input");
	task2::run(INPUT);
}
