use std::collections::HashSet;

const FIELD_MIN: Position = Position { x: -25, y: -25 };
const FIELD_MAX: Position = Position { x: 25, y: 25 };

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
	rope: [Position; 10],
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
		self.rope[0].step(direction);

		for head_index in 0..9 {
			let tail_index = head_index + 1;
			let head = self.rope[head_index];
			let tail = &mut self.rope[tail_index];

			let (x_dist, y_dist) = head.dist(*tail);
			if x_dist == 0 && y_dist > 1 {
				if head.y > tail.y {
					tail.y += 1;
				} else {
					tail.y -= 1;
				}
			} else if x_dist > 1 && y_dist == 0 {
				if head.x > tail.x {
					tail.x += 1;
				} else {
					tail.x -= 1;
				}
			} else if x_dist > 1 || y_dist > 1 {
				if head.x > tail.x {
					tail.x += 1;
				} else {
					tail.x -= 1;
				}

				if head.y > tail.y {
					tail.y += 1;
				} else {
					tail.y -= 1;
				}
			}
		}

		self.visited.insert(self.rope[9]);
	}

	pub fn print(&self, min: Position, max: Position) {
		for y in min.y..=max.y {
			'outer: for x in min.x..=max.x {
				let pos = Position { x, y };
				if pos == self.rope[0] {
					print!("H");
					continue;
				}
				for i in 1..=9 {
					if pos == self.rope[i] {
						print!("{i}");
						continue 'outer;
					}
				}
				if self.visited.contains(&pos) {
					print!("#");
				} else {
					print!(".");
				}
			}
			println!();
		}
	}

	pub fn print_visits(&self, min: Position, max: Position) {
		for y in min.y..=max.y {
			for x in min.x..=max.x {
				let pos = Position { x, y };
				if self.visited.contains(&pos) {
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
fn test_sample() {
	assert_eq!(run(super::SAMPLE2), 36);
}

#[test]
fn test_input() {
	assert_eq!(run(super::INPUT), 2458);
}
