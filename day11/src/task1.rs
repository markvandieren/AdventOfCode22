use crate::{common::*, parse};

pub fn run(s: &str) -> u128 {
	let mut monkeys = parse::monkeys(s);

	for _ in 0..20 {
		perform_round(&mut monkeys);
	}

	calculate_monkey_business(&monkeys)
}

fn perform_round(monkeys: &mut [Monkey]) {
	for i in 0..monkeys.len() {
		while let Some((worry_level, target_monkey)) = do_business(&mut monkeys[i]) {
			monkeys[target_monkey].items.push(worry_level);
		}
	}
}

fn do_business(monkey: &mut Monkey) -> Option<(u128, usize)> {
	monkey.get_worry().map(|worry| {
		let worry = worry / 3;
		monkey.test(worry)
	})
}

#[test]
fn test() {
	let mut monkeys = parse::monkeys(super::SAMPLE);
	perform_round(&mut monkeys);
	assert_eq!(monkeys[0].items, [20, 23, 27, 26]);
	assert_eq!(monkeys[1].items, [2080, 25, 167, 207, 401, 1046]);
	assert_eq!(monkeys[2].items, []);
	assert_eq!(monkeys[3].items, []);

	for _ in 0..19 {
		perform_round(&mut monkeys);
	}
	assert_eq!(monkeys[0].inspection_count, 101);
	assert_eq!(monkeys[1].inspection_count, 95);
	assert_eq!(monkeys[2].inspection_count, 7);
	assert_eq!(monkeys[3].inspection_count, 105);
	assert_eq!(calculate_monkey_business(&monkeys), 10_605);
}
