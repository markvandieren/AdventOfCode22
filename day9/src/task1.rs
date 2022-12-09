use std::collections::HashSet;

pub fn run(s: &str) -> usize {
	let mut state = State::new();

	for (direction, amount) in s.lines().map(|line| {
		let (a, b) = line.split_once(' ').unwrap();
		(Direction::from(a), b.parse::<i32>().unwrap())
	}) {
		state.apply_move(direction, amount);
	}

	state.visited.len()
}

#[derive(Default)]
struct State {
	head: Position,
	tail: Position,
	visited: HashSet<Position>,
}

impl State {
	pub fn new() -> Self {
		let mut state = State::default();
		state.visited.insert(Position::default());
		state
	}

	pub fn apply_move(&mut self, direction: Direction, amount: i32) {
		for _ in 0..amount {
			self.step(direction);
		}
	}

	fn step(&mut self, direction: Direction) {
		self.head.step(direction);

		let (x_dist, y_dist) = self.head.dist(self.tail);
		if x_dist > 1 || y_dist > 1 {
			if (x_dist == 1 && y_dist > 1) || (x_dist > 1 && y_dist == 1) {
				if self.head.x > self.tail.x {
					self.tail.x += 1;
				} else {
					self.tail.x -= 1;
				}

				if self.head.y > self.tail.y {
					self.tail.y += 1;
				} else {
					self.tail.y -= 1;
				}
			} else {
				self.tail.step(direction);
			}

			self.visited.insert(self.tail);
		}
	}

	pub fn print(&self, min: Position, max: Position) {
		for y in min.y..=max.y {
			for x in min.x..=max.x {
				let pos = Position { x, y };
				if pos == self.head {
					print!("H");
				} else if pos == self.tail {
					print!("T");
				} else if self.visited.contains(&pos) {
					print!("#");
				} else {
					print!(".");
				}
			}
			println!();
		}
	}
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
	x: i32,
	y: i32,
}

impl Position {
	pub fn dist(self, other: Self) -> (i32, i32) {
		((self.x - other.x).abs(), (self.y - other.y).abs())
	}

	pub fn step(&mut self, direction: Direction) {
		match direction {
			Direction::Up => {
				self.y -= 1;
			}
			Direction::Down => {
				self.y += 1;
			}
			Direction::Left => {
				self.x -= 1;
			}
			Direction::Right => {
				self.x += 1;
			}
		}
	}
}

#[derive(Debug, Clone, Copy)]
enum Direction {
	Up,
	Down,
	Left,
	Right,
}

impl From<&str> for Direction {
	fn from(s: &str) -> Self {
		match s {
			"U" => Self::Up,
			"D" => Self::Down,
			"L" => Self::Left,
			"R" => Self::Right,
			_ => panic!(),
		}
	}
}

#[derive(Debug, Clone, Copy)]
enum UpDown {
	Up,
	Down,
}

#[derive(Debug, Clone, Copy)]
enum LeftRight {
	Left,
	Right,
}

#[test]
fn test() {
	use super::*;

	assert_eq!(run(SAMPLE), 13);
	assert_eq!(run(INPUT), 6271);
}
