#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Vec2 {
	pub x: i32,
	pub y: i32,
}

impl Vec2 {
	#[must_use]
	pub const fn new(x: i32, y: i32) -> Self {
		Self { x, y }
	}

	#[must_use]
	pub fn is_in_range(self, limit: Vec2) -> bool {
		self.x >= 0 && self.y >= 0 && self.x < limit.x && self.y < limit.y
	}

	#[must_use]
	#[allow(clippy::cast_sign_loss)]
	pub fn index(self, width: usize) -> usize {
		assert!(self.x >= 0);
		assert!(self.y >= 0);
		self.y as usize * width + self.x as usize
	}

	pub fn distance(self, other: Self) -> u32 {
		self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
	}
}

impl std::ops::Add for Vec2 {
	type Output = Vec2;

	fn add(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
	}
}

impl std::ops::Sub for Vec2 {
	type Output = Vec2;

	fn sub(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
		}
	}
}

impl std::fmt::Display for Vec2 {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}, {})", self.x, self.y)
	}
}
