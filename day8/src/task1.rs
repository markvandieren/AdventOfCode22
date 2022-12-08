use super::*;

pub fn run(s: &str) -> usize {
	let grid = common::parse_grid(s);
	let width = grid[0].len();
	let height = grid.len();

	let mut count = width * 2 + height * 2 - 4;
	for y in 1..height - 1 {
		for x in 1..width - 1 {
			if is_visible(&grid, x, y) {
				count += 1;
			}
		}
	}
	count
}

fn is_visible(grid: &[Vec<u32>], x: usize, y: usize) -> bool {
	let width = grid[0].len();
	let height = grid.len();
	let tree_height = grid[y][x];

	let left = (0..x).map(|i| grid[y][i]).all(|h| h < tree_height);
	let right = (x + 1..width).map(|i| grid[y][i]).all(|h| h < tree_height);
	let top = (0..y).map(|i| grid[i][x]).all(|h| h < tree_height);
	let bottom = (y + 1..height).map(|i| grid[i][x]).all(|h| h < tree_height);
	left || right || top || bottom
}

#[test]
fn test() {
	assert_eq!(run(SAMPLE), 21);
	assert_eq!(run(INPUT), 1733);
}
