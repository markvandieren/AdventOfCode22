#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::wildcard_imports)]
#![allow(dead_code)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_wrap)]

const INPUT: &str = include_str!("../data/input.txt");
const SAMPLE: &str = include_str!("../data/sample.txt");

fn main() {
	let sample = run(SAMPLE);
	println!("1. Sample: {sample}");

	let input = run(INPUT);
	println!("1. Input: {input}");

	let sample = run2(SAMPLE);
	println!("2. Sample: {sample}");

	let input = run2(INPUT);
	println!("2. Input: {input}");
}

#[test]
fn test() {
	assert_eq!(run(SAMPLE), 31);
	assert_eq!(run(INPUT), 456);
	assert_eq!(run2(SAMPLE), 29);
	assert_eq!(run2(INPUT), 454);
}

fn run(s: &str) -> usize {
	let (start, end, map) = Map::new(s);
	BfsSolver::run(&map, start, end)
	// dfs_solve(&map, start, end, 0, &[])
}

fn run2(s: &str) -> usize {
	let (_start, end, map) = Map::new(s);

	let mut best = usize::MAX;
	for (y, row) in map.h.iter().enumerate() {
		for (x, c) in row.iter().enumerate() {
			if *c == 0 {
				let path = BfsSolver::run(&map, Pos::new(x as isize, y as isize), end);
				best = best.min(path);
			}
		}
	}
	best
}

struct Map {
	h: Vec<Vec<u8>>,
}

impl Map {
	pub fn new(s: &str) -> (Pos, Pos, Self) {
		let mut rows = Vec::new();
		let mut start = Pos::new(0, 0);
		let mut end = Pos::new(0, 0);

		for line in s.lines() {
			let mut cols = Vec::new();
			for c in line.chars() {
				if c == 'S' {
					start = Pos::new(cols.len() as isize, rows.len() as isize);
					cols.push(calc_elevation('a'));
				} else if c == 'E' {
					end = Pos::new(cols.len() as isize, rows.len() as isize);
					cols.push(calc_elevation('z'));
				} else {
					cols.push(calc_elevation(c));
				}
			}
			rows.push(cols);
		}

		(start, end, Map { h: rows })
	}

	pub fn is_in_range(&self, pos: Pos) -> bool {
		pos.is_in_range(Pos::new(self.h[0].len() as isize, self.h.len() as isize))
	}

	pub fn width(&self) -> usize {
		self.h[0].len()
	}

	pub fn height(&self) -> usize {
		self.h.len()
	}
}

impl std::ops::Index<Pos> for Map {
	type Output = u8;

	#[allow(clippy::cast_sign_loss)]
	fn index(&self, pos: Pos) -> &Self::Output {
		assert!(pos.x >= 0);
		assert!(pos.y >= 0);
		assert!(self.h[0].len() > pos.x as usize);
		assert!(self.h.len() > pos.y as usize);
		&self.h[pos.y as usize][pos.x as usize]
	}
}

//https://www.youtube.com/watch?v=oDqjPvD54Ss
struct BfsSolver;

impl BfsSolver {
	fn run(map: &Map, start: Pos, end: Pos) -> usize {
		let prev = Self::solve(map, start);
		if let Some(path) = Self::reconstruct_path(map, start, end, &prev) {
			return path.len() - 1;
		}
		usize::MAX
	}

	fn reconstruct_path(map: &Map, start: Pos, end: Pos, prev: &[Option<Pos>]) -> Option<Vec<Pos>> {
		let mut path = Vec::new();
		let mut at = Some(end);
		while let Some(v) = at {
			path.push(v);
			at = prev[v.index(map.width())];
		}

		path.reverse();

		if path[0] == start {
			Some(path)
		} else {
			None
		}
	}

	fn solve(map: &Map, start: Pos) -> Vec<Option<Pos>> {
		let mut queue = std::collections::VecDeque::new();
		queue.push_back(start);

		let mut visited = vec![false; map.width() * map.height()];
		visited[start.index(map.width())] = true;

		let mut prev: Vec<Option<Pos>> = vec![None; map.width() * map.height()];

		while let Some(node) = queue.pop_front() {
			for i in 0..4 {
				let next = node + Pos::NEIGHBORS[i];
				if map.is_in_range(next) && map[next] <= map[node] + 1 {
					let index = next.index(map.width());
					if !visited[index] {
						queue.push_back(next);
						visited[index] = true;
						prev[index] = Some(node);
					}
				}
			}
		}
		prev
	}
}

fn dfs_solve(map: &Map, start: Pos, end: Pos, steps: usize, path: &[Pos]) -> usize {
	let start_h = map[start];
	let mut best_result = usize::MAX;
	for i in 0..4 {
		let neighbor = start + Pos::NEIGHBORS[i];
		if !map.is_in_range(neighbor) || path.contains(&neighbor) {
			continue;
		}
		let neighbor_h = map[neighbor];

		if neighbor == end && neighbor_h <= start_h + 1 {
			let steps = steps + 1;
			if steps < best_result {
				best_result = steps;
			}
		}
	}
	if best_result == usize::MAX {
		for i in 0..4 {
			let neighbor = start + Pos::NEIGHBORS[i];
			if !map.is_in_range(neighbor) || path.contains(&neighbor) {
				continue;
			}
			let neighbor_h = map[neighbor];
			if neighbor_h <= start_h + 1 {
				let mut path = path.to_vec();
				path.push(neighbor);
				let result = dfs_solve(map, neighbor, end, steps + 1, &path);
				best_result = best_result.min(result);
			}
		}
	}
	best_result
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
struct Pos {
	x: isize,
	y: isize,
}

impl Pos {
	const LEFT: Self = Pos::new(-1, 0);
	const RIGHT: Self = Pos::new(1, 0);
	const TOP: Self = Pos::new(0, -1);
	const BOTTOM: Self = Pos::new(0, 1);
	const NEIGHBORS: [Pos; 4] = [Self::LEFT, Self::RIGHT, Self::TOP, Self::BOTTOM];

	#[must_use]
	pub const fn new(x: isize, y: isize) -> Self {
		Self { x, y }
	}

	pub fn is_in_range(self, limit: Pos) -> bool {
		self.x >= 0 && self.y >= 0 && self.x < limit.x && self.y < limit.y
	}

	pub fn index(self, width: usize) -> usize {
		assert!(self.x >= 0);
		assert!(self.y >= 0);
		self.y as usize * width + self.x as usize
	}
}

impl std::ops::Add for Pos {
	type Output = Pos;

	fn add(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
	}
}

impl std::fmt::Display for Pos {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}, {})", self.x, self.y)
	}
}

fn calc_elevation(c: char) -> u8 {
	c as u8 - b'a'
}
