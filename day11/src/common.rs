pub struct Monkey {
	pub items: Vec<u128>,
	pub op: Operation,
	pub test: u128,
	pub if_true: usize,
	pub if_false: usize,
	pub inspection_count: u128,
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
	Add(Value),
	Mul(Value),
}

#[derive(Debug, Clone, Copy)]
pub enum Value {
	Num(u128),
	Old,
}

impl Monkey {
	pub fn get_worry(&mut self) -> Option<u128> {
		if self.items.is_empty() {
			None
		} else {
			self.inspection_count += 1;
			Some(self.op.apply(self.items.remove(0)))
		}
	}

	pub fn test(&self, worry: u128) -> (u128, usize) {
		let next_monkey = if worry % self.test == 0 {
			self.if_true
		} else {
			self.if_false
		};
		(worry, next_monkey)
	}
}

pub fn calculate_monkey_business(monkeys: &[Monkey]) -> u128 {
	let mut inspection_counts: Vec<_> = monkeys
		.iter()
		.map(|monkey| monkey.inspection_count)
		.collect();
	inspection_counts.sort_unstable();
	inspection_counts.iter().rev().take(2).product()
}

impl Operation {
	pub fn apply(self, worry: u128) -> u128 {
		match self {
			Operation::Add(num) => match num {
				Value::Num(num) => worry + num,
				Value::Old => worry + worry,
			},
			Operation::Mul(num) => match num {
				Value::Num(num) => worry * num,
				Value::Old => worry * worry,
			},
		}
	}
}
