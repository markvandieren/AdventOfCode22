pub fn run(s: &str) -> i64 {
	let mut state = State::default();
	for line in s.lines() {
		if let Some((cmd, num)) = line.split_once(' ') {
			assert_eq!(cmd, "addx");
			let num = num.parse::<i64>().unwrap();
			state.addx(num);
		} else {
			assert_eq!(line, "noop");
			state.noop();
		}
	}
	state.accumulator
}

struct State {
	cycle: i64,
	register_x: i64,
	accumulator: i64,
}

impl State {
	fn tick_cycle(&mut self) {
		self.cycle += 1;
		if self.cycle == 20 || (self.cycle > 20 && (self.cycle - 20) % 40 == 0) {
			self.accumulator += self.signal_strength();
			println!(
				"Accumulating cycle {}: {} ({})",
				self.cycle,
				self.signal_strength(),
				self.accumulator
			);
		}
	}

	pub fn noop(&mut self) {
		self.tick_cycle();
	}

	pub fn addx(&mut self, amount: i64) {
		self.tick_cycle();
		self.tick_cycle();
		self.register_x += amount;
	}

	pub fn signal_strength(&self) -> i64 {
		self.cycle * self.register_x
	}
}

impl Default for State {
	fn default() -> Self {
		Self {
			cycle: 0,
			register_x: 1,
			accumulator: 0,
		}
	}
}

#[test]
fn test() {
	assert_eq!(run(super::SAMPLE), 13140);
	assert_eq!(run(super::INPUT), 16060);
}
