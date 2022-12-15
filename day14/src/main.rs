#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::wildcard_imports)]
#![allow(dead_code)]
#![allow(clippy::cast_sign_loss)]

mod parse;
mod pos;
mod task1;

const INPUT: &str = include_str!("../data/input.txt");
const SAMPLE: &str = include_str!("../data/sample.txt");

use crate::pos::Pos;

fn main() {
	let sample = run(SAMPLE);
	println!("1. Sample: {sample}");
	assert_eq!(sample, 24);

	let input = run(INPUT);
	println!("1. Input: {input}");
	assert_eq!(input, 799);

	let sample = run2(SAMPLE);
	println!("2. Sample: {sample}");
	assert_eq!(sample, 93);

	let input = run2(INPUT);
	println!("2. Input: {input}");
	assert_eq!(input, 29076);
}

const SAND_SOURCE: Pos = Pos::new(500, 0);

fn run(s: &str) -> usize {
	let paths: Vec<_> = s.lines().map(parse::path).collect();
	println!("Paths: {paths:?}");
	println!();

	let (min, max) = calc_bounds(&paths);

	let mut grid = Grid::new(min, max);
	set_paths(&mut grid, &paths);

	println!("Initial grid:");
	println!("{grid}");

	let mut count = 0;
	let mut res = SimResult::Settled;
	while res == SimResult::Settled {
		grid.set(SAND_SOURCE, Cell::Sand);
		res = grid.simulate(SAND_SOURCE);
		count += 1;
		// println!("Simulated grid: {res:?}");
		// println!("{grid}");
	}

	println!("Final grid: {count} steps");
	println!("{grid}");

	count - 1
}

fn run2(s: &str) -> usize {
	let paths: Vec<_> = s.lines().map(parse::path).collect();
	println!("Paths: {paths:?}");
	println!();

	let (min, max) = calc_bounds(&paths);

	let mut grid = Grid::new(min - Pos::new(200, 0), max + Pos::new(200, 1));
	set_paths(&mut grid, &paths);

	println!("Initial grid:");
	println!("{grid}");

	let mut count = 0;
	loop {
		if let Some(Cell::Sand) = grid.get(SAND_SOURCE) {
			break;
		}
		grid.set(SAND_SOURCE, Cell::Sand);
		grid.simulate2(SAND_SOURCE);
		count += 1;
		// println!("Simulated grid: {res:?}");
		// println!("{grid}");
	}

	println!("Final grid: {count} steps");
	println!("{grid}");

	count
}

fn set_paths(grid: &mut Grid, paths: &[Vec<Pos>]) {
	for path in paths {
		for i in 0..path.len() - 1 {
			let from = path[i];
			let to = path[i + 1];
			grid.set_line(from, to, Cell::Rock);
		}
	}
}

struct Grid {
	cells: Vec<Cell>,
	min: Pos,
	max: Pos,
	range: Pos,
}

impl Grid {
	pub fn new(min: Pos, max: Pos) -> Self {
		let range = max - min;
		// println!("Grid::new([{min} - {max}] = {range})");
		Self {
			cells: vec![Cell::Air; (range.x * range.y) as usize],
			min,
			max,
			range,
		}
	}

	pub fn is_in_bounds(&self, p: Pos) -> bool {
		p > self.min && p < self.max
	}

	pub fn get(&self, p: Pos) -> Option<Cell> {
		if self.is_in_bounds(p) {
			let local = p - self.min;
			let index = local.index(self.range.x as usize);
			self.cells.get(index).copied()
		} else {
			None
		}
	}

	pub fn get_expect(&self, p: Pos) -> Cell {
		let local = p - self.min;
		let index = local.index(self.range.x as usize);
		*self.cells.get(index).unwrap()
	}

	pub fn set(&mut self, p: Pos, v: Cell) {
		assert!(self.is_in_bounds(p));
		let local = p - self.min;
		let index = local.index(self.range.x as usize);
		// println!("Grid::set({p}, {v:?}): {local}={index}");
		*self.cells.get_mut(index).unwrap() = v;
	}

	pub fn swap(&mut self, from: Pos, to: Pos) {
		let a = self.get(from).unwrap();
		let b = self.get(to).unwrap();
		self.set(from, b);
		self.set(to, a);
	}

	pub fn set_line(&mut self, mut from: Pos, mut to: Pos, v: Cell) {
		if from.x == to.x {
			if from.y > to.y {
				std::mem::swap(&mut from.y, &mut to.y);
			}
			for y in from.y..=to.y {
				self.set(Pos::new(from.x, y), v);
			}
		} else if from.y == to.y {
			if from.x > to.x {
				std::mem::swap(&mut from.x, &mut to.x);
			}
			for x in from.x..=to.x {
				self.set(Pos::new(x, from.y), v);
			}
		} else {
			panic!("Diagonal lines not supported")
		}
	}

	pub fn simulate(&mut self, p: Pos) -> SimResult {
		let v = self.get(p).unwrap();
		assert_eq!(v, Cell::Sand);
		// println!("simulate({p}:{v})");

		let mut from = p;
		let mut to = from.bottom();
		loop {
			if !self.is_in_bounds(to) {
				return SimResult::OutOfBounds;
			}
			let val = self.get(to);
			if val.is_none() {
				return SimResult::OutOfBounds;
			}
			let val = val.unwrap();
			// println!("target {to}:{val}");

			match val {
				Cell::Air => {
					self.swap(from, to);
					from = to;
					to = from.bottom();
				}
				Cell::Rock | Cell::Sand => {
					let left = to.left();
					let right = to.right();

					if !self.is_in_bounds(left) {
						return SimResult::OutOfBounds;
					} else if let Some(Cell::Air) = self.get(left) {
						self.swap(from, left);
						from = left;
						to = from.bottom();
					} else if !self.is_in_bounds(right) {
						return SimResult::OutOfBounds;
					} else if let Some(Cell::Air) = self.get(right) {
						self.swap(from, right);
						from = right;
						to = from.bottom();
					} else {
						return SimResult::Settled;
					}
				}
			}
		}
	}

	pub fn simulate2(&mut self, p: Pos) -> SimResult2 {
		let v = self.get(p).unwrap();
		assert_eq!(v, Cell::Sand);
		// println!("simulate({p}:{v})");

		let mut from = p;
		let mut to = from.bottom();
		while let Some(val) = self.get(to) {
			// println!("target {to}:{val}");

			match val {
				Cell::Air => {
					self.swap(from, to);
					from = to;
					to = from.bottom();
				}
				Cell::Rock | Cell::Sand => {
					let left = to.left();
					let right = to.right();

					if let Some(Cell::Air) = self.get(left) {
						self.swap(from, left);
						from = left;
						to = from.bottom();
					} else if let Some(Cell::Air) = self.get(right) {
						self.swap(from, right);
						from = right;
						to = from.bottom();
					} else {
						break;
					}
				}
			}
		}

		SimResult2::Settled
	}
}

#[derive(Debug, PartialEq, Eq)]
enum SimResult {
	Settled,
	OutOfBounds,
}

#[derive(Debug, PartialEq, Eq)]
enum SimResult2 {
	Settled,
	Blocked,
}

impl std::fmt::Display for Grid {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for y in 0..self.range.y {
			for x in 0..self.range.x {
				write!(
					f,
					"{}",
					self.cells
						.get(Pos::new(x, y).index(self.range.x as usize))
						.unwrap()
				)?;
			}
			writeln!(f)?;
		}
		Ok(())
	}
}

fn calc_bounds(paths: &[Vec<Pos>]) -> (Pos, Pos) {
	let mut min = SAND_SOURCE;
	let mut max = SAND_SOURCE;
	for path in paths {
		for pos in path {
			min.x = min.x.min(pos.x);
			min.y = min.y.min(pos.y);

			max.x = max.x.max(pos.x);
			max.y = max.y.max(pos.y);
		}
	}

	max = max + Pos::new(1, 1);

	(min, max)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
	Air,
	Rock,
	Sand,
}

impl std::fmt::Display for Cell {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Cell::Air => write!(f, "."),
			Cell::Rock => write!(f, "#"),
			Cell::Sand => write!(f, "+"),
		}
	}
}
