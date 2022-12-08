use super::*;

pub fn run(s: &str) -> usize {
	let grid = common::parse_grid(s);
	let width = grid[0].len();
	let height = grid.len();

	let mut max_score = 0;
	for y in 0..height {
		for x in 0..width {
			let score = get_score(&grid, x, y);
			max_score = max_score.max(score);
		}
	}
	max_score
}

fn get_score(grid: &[Vec<u32>], x: usize, y: usize) -> usize {
	let h = grid[y][x];
	let width = grid[0].len();
	let height = grid.len();

	let left_score = calc_score(h, (0..x).rev().map(|i| grid[y][i]));
	let right_score = calc_score(h, (x + 1..width).map(|i| grid[y][i]));
	let top_score = calc_score(h, (0..y).rev().map(|i| grid[i][x]));
	let bottom_score = calc_score(h, (y + 1..height).map(|i| grid[i][x]));

	left_score * right_score * top_score * bottom_score
}

fn calc_score(tree_height: u32, i: impl Iterator<Item = u32>) -> usize {
	let mut score = 0;
	let mut min = 0;
	for h in i {
		score += 1;
		if handle(h, &mut min, tree_height) {
			break;
		}
	}
	score
}

fn handle(h: u32, min: &mut u32, target_tree: u32) -> bool {
	*min = (*min).max(h);
	h >= target_tree
}

#[test]
fn test() {
	let grid = common::parse_grid(SAMPLE);
	assert_eq!(get_score(&grid, 2, 1), 4);
	assert_eq!(grid[3][2], 5);
	assert_eq!(get_score(&grid, 2, 3), 8);
	assert!(get_score(&grid, 3, 2) <= 8);
	assert_eq!(run(SAMPLE), 8);
	assert_eq!(run(INPUT), 284_648);
}
