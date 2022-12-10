pub fn run(s: &str) {
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
	state.display_screen();
}

struct State {
	cycle: i64,
	register_x: i64,
	screen: Vec<String>,
}

// fn format_row(pos: i64) -> String {
//	use std::fmt::Write;
// 	let mut out = String::new();
// 	for i in 0..40 {
// 		if (pos - i).abs() < 2 {
// 			write!(out, "#").unwrap();
// 		} else {
// 			write!(out, ".").unwrap();
// 		}
// 	}
// 	out
// }

impl State {
	fn tick_cycle(&mut self) {
		self.cycle += 1;
		self.tick_screen();
	}

	fn tick_screen(&mut self) {
		// let row = self.cycle / 40;
		//let s = format_row(self.register_x);
		let cursor = (self.cycle - 1) % 40;

		if cursor == 0 {
			self.screen.push(String::new());
		}
		let screen_buffer = self.screen.last_mut().unwrap();
		if (self.register_x - cursor).abs() < 2 {
			screen_buffer.push('#');
		} else {
			screen_buffer.push('.');
		}
		// println!(
		// 	"[cycle {:>3}][x {:>2}][row {row}][cursor {cursor}] {s}",
		// 	self.cycle, self.register_x
		// );
		// println!("Screen: [row {row}] {screen_buffer}");
	}

	pub fn display_screen(&self) {
		for s in &self.screen {
			println!("{s}");
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
}

impl Default for State {
	fn default() -> Self {
		Self {
			cycle: 0,
			register_x: 1,
			screen: Vec::new(),
		}
	}
}
