#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pos {
	pub x: i32,
	pub y: i32,
}

impl Pos {
	const LEFT: Self = Pos::new(-1, 0);
	const RIGHT: Self = Pos::new(1, 0);
	const TOP: Self = Pos::new(0, -1);
	const BOTTOM: Self = Pos::new(0, 1);
	const NEIGHBORS: [Pos; 4] = [Self::LEFT, Self::RIGHT, Self::TOP, Self::BOTTOM];

	#[must_use]
	pub const fn new(x: i32, y: i32) -> Self {
		Self { x, y }
	}

	#[must_use]
	pub fn is_in_range(self, limit: Pos) -> bool {
		self.x >= 0 && self.y >= 0 && self.x < limit.x && self.y < limit.y
	}

	#[must_use]
	#[allow(clippy::cast_sign_loss)]
	pub fn index(self, width: usize) -> usize {
		assert!(self.x >= 0);
		assert!(self.y >= 0);
		self.y as usize * width + self.x as usize
	}

	#[must_use]
	pub fn bottom(self) -> Self {
		Self {
			x: self.x,
			y: self.y + 1,
		}
	}

	#[must_use]
	pub fn left(self) -> Self {
		Self {
			x: self.x - 1,
			y: self.y,
		}
	}

	#[must_use]
	pub fn right(self) -> Self {
		Self {
			x: self.x + 1,
			y: self.y,
		}
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

impl std::ops::Sub for Pos {
	type Output = Pos;

	fn sub(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
		}
	}
}

impl std::fmt::Display for Pos {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}, {})", self.x, self.y)
	}
}
