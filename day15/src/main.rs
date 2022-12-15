#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::wildcard_imports)]
#![allow(dead_code)]
#![allow(clippy::cast_sign_loss)]

const INPUT: &str = include_str!("../data/input.txt");
const SAMPLE: &str = include_str!("../data/sample.txt");

mod parse;
mod vec2;

use crate::vec2::Vec2;
use std::ops::Range;

fn main() {
	let sample = run(SAMPLE, 10, -2, 26);
	println!("1. Sample: {sample}");
	assert_eq!(sample, 26);

	let input = run(INPUT, 2_000_000, -100_000_000, 100_000_000);
	println!("1. Input: {input}");
	assert_eq!(input, 5_127_797);

	let sample = run2(SAMPLE, 20).unwrap();
	println!("2. Sample: {sample}");
	assert_eq!(sample, 56_000_011);

	let input = run2(INPUT, 4_000_000).unwrap();
	println!("2. Input: {input}");
	assert_eq!(input, 12_518_502_636_475);
}

fn run(s: &str, y_level: i32, x_min: i32, x_max: i32) -> usize {
	let positions = parse::read_positions(s);

	let positions: Vec<_> = positions
		.iter()
		.map(|(sensor, beacon)| (*sensor, *beacon, sensor.distance(*beacon)))
		.collect();

	let mut count = 0;
	let y = y_level;
	'outer: for x in x_min..=x_max {
		let p = Vec2::new(x, y);
		for (_, beacon, _) in &positions {
			if p == *beacon {
				continue 'outer;
			}
		}
		for (sensor, _beacon, distance) in &positions {
			if p.distance(*sensor) <= *distance {
				count += 1;
				break;
			}
		}
	}
	count
}

fn run2(s: &str, field_max: i32) -> Option<u64> {
	let positions = parse::read_positions(s);
	for y in 0..field_max {
		if let Some(res) = check_row(&positions, y, field_max) {
			return Some(res);
		}
	}
	None
}

fn check_row(positions: &Vec<(Vec2, Vec2)>, y: i32, field_max: i32) -> Option<u64> {
	let beacon_free_ranges = {
		let mut out = find_beacon_free_ranges(positions, y, field_max);
		out.sort_unstable_by_key(|a| a.start);
		out
	};

	let combined_ranges = combine_ranges(&beacon_free_ranges);
	ensure_entire_range_is_covered(&combined_ranges, field_max);

	if combined_ranges.len() > 1 {
		let x = combined_ranges.first().unwrap().end + 1;
		let res = calc_frequency(x, y);
		return Some(res);
	}
	None
}

fn calc_frequency(x: i32, y: i32) -> u64 {
	4_000_000 * x as u64 + y as u64
}

fn ensure_entire_range_is_covered(combined_ranges: &[Range<i32>], field_max: i32) {
	assert!(combined_ranges.first().unwrap().start == 0);
	assert!(combined_ranges.last().unwrap().end == field_max);
}

/// Find what ranges are guaranteed to not have a beacon in them
fn find_beacon_free_ranges(
	positions: &Vec<(Vec2, Vec2)>,
	y: i32,
	field_max: i32,
) -> Vec<Range<i32>> {
	let mut out: Vec<Range<i32>> = Vec::new();
	for (sensor, beacon) in positions {
		let distance = sensor.distance(*beacon);
		let dy = sensor.y.abs_diff(y);

		if dy < distance {
			let dx = i32::try_from(distance - dy).unwrap();
			let min = (sensor.x - dx).max(0);
			let max = (sensor.x + dx).min(field_max);
			out.push(min..max);
		}
	}
	out
}

fn combine_ranges(beacon_free_ranges: &[Range<i32>]) -> Vec<Range<i32>> {
	let mut combined_ranges: Vec<Range<i32>> = Vec::new();
	combined_ranges.push(beacon_free_ranges[0].clone());
	for range in beacon_free_ranges.iter().skip(1) {
		let top = combined_ranges.last().unwrap();

		if range.start > top.end {
			combined_ranges.push(range.clone());
		} else {
			*combined_ranges.last_mut().unwrap() = top.start..range.end.max(top.end);
		}
	}
	combined_ranges
}
