use crate::{common::*, parse};

pub fn run(s: &str) -> u128 {
	let mut monkeys = parse::monkeys(s);

	let mut divisor = 1;
	for monkey in &monkeys {
		divisor *= monkey.test;
	}

	let mut round = 0;
	for _ in 0..10_000 {
		perform_round(&mut round, &mut monkeys, divisor);
	}

	calculate_monkey_business(&monkeys)
}

fn perform_round(round_index: &mut u64, monkeys: &mut [Monkey], divisor: u128) {
	*round_index += 1;
	for i in 0..monkeys.len() {
		while let Some((worry_level, target_monkey)) = do_business(&mut monkeys[i], divisor) {
			monkeys[target_monkey].items.push(worry_level);
		}
	}
}

fn do_business(monkey: &mut Monkey, divisor: u128) -> Option<(u128, usize)> {
	monkey.get_worry().map(|worry| {
		let worry = worry % divisor;
		monkey.test(worry)
	})
}

#[test]
fn test() {
	fn assert_state(monkeys: &[Monkey], counts: [u128; 4]) {
		for (monkey, count) in monkeys.iter().zip(counts.iter()) {
			assert_eq!(monkey.inspection_count, *count);
		}
	}

	let mut monkeys = parse::monkeys(super::SAMPLE);

	let mut divisor = 1;
	for monkey in &monkeys {
		divisor *= monkey.test;
	}

	let mut round = 0;
	perform_round(&mut round, &mut monkeys, divisor);
	assert_state(&monkeys, [2, 4, 3, 6]);

	while round < 20 {
		perform_round(&mut round, &mut monkeys, divisor);
	}
	assert_state(&monkeys, [99, 97, 8, 103]);

	while round < 1000 {
		perform_round(&mut round, &mut monkeys, divisor);
	}
	assert_state(&monkeys, [5204, 4792, 199, 5192]);

	while round < 10_000 {
		perform_round(&mut round, &mut monkeys, divisor);
	}
	assert_state(&monkeys, [52166, 47830, 1938, 52013]);

	assert_eq!(calculate_monkey_business(&monkeys), 2_713_310_158);
}
