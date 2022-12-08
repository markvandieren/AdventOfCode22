pub fn parse_grid(s: &str) -> Vec<Vec<u32>> {
	let mut grid = Vec::new();
	for row in s
		.lines()
		.map(|line| line.chars().map(|c| c.to_digit(10).unwrap()))
	{
		grid.push(row.collect::<Vec<_>>());
	}
	grid
}
