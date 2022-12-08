#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
#![allow(dead_code)]

const INPUT: &str = include_str!("../data/input.txt");
const SAMPLE: &str = include_str!("../data/sample.txt");

fn main() {
	assert_eq!(run1(SAMPLE), 15);
	println!("score1: {}", run1(INPUT));
	assert_eq!(run2(SAMPLE), 12);
	println!("score2: {}", run2(INPUT));
}

fn run1(input: &str) -> u64 {
	let mut sum = 0;
	for line in input.lines() {
		let (opponent, you) = line.split_once(' ').unwrap();
		let opponent = Move::from(opponent.chars().next().unwrap());
		let you = Move::from(you.chars().next().unwrap());
		//println!("{opponent:?} vs {you:?} -> {} points", score(you, opponent));
		sum += u64::from(score(you, opponent));
	}

	sum
}

fn run2(input: &str) -> u64 {
	let mut sum = 0;
	for line in input.lines() {
		let (opponent, outcome) = line.split_once(' ').unwrap();
		let opponent = Move::from(opponent.chars().next().unwrap());
		let outcome = Outcome::from(outcome.chars().next().unwrap());
		let you = calculate_move(opponent, outcome);
		let score = score(you, opponent);
		//println!("{opponent:?} vs {you:?} with outcome {outcome:?} -> {score}");
		sum += u64::from(score);
	}

	sum
}

#[derive(Debug, Clone, Copy)]
enum Move {
	Rock,
	Paper,
	Scissors,
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
	Win,
	Draw,
	Lose,
}

impl Outcome {
	pub fn value(self) -> u8 {
		match self {
			Self::Win => 6,
			Self::Draw => 3,
			Self::Lose => 0,
		}
	}
}

impl From<char> for Outcome {
	fn from(c: char) -> Self {
		match c {
			'X' => Self::Lose,
			'Y' => Self::Draw,
			'Z' => Self::Win,
			_ => panic!(),
		}
	}
}

impl Move {
	pub fn value(self) -> u8 {
		match self {
			Self::Rock => 1,
			Self::Paper => 2,
			Self::Scissors => 3,
		}
	}

	pub fn battle(self, other: Self) -> Outcome {
		match self {
			Move::Rock => match other {
				Move::Rock => Outcome::Draw,
				Move::Paper => Outcome::Lose,
				Move::Scissors => Outcome::Win,
			},
			Move::Paper => match other {
				Move::Rock => Outcome::Win,
				Move::Paper => Outcome::Draw,
				Move::Scissors => Outcome::Lose,
			},
			Move::Scissors => match other {
				Move::Rock => Outcome::Lose,
				Move::Paper => Outcome::Win,
				Move::Scissors => Outcome::Draw,
			},
		}
	}
}

impl From<char> for Move {
	fn from(c: char) -> Self {
		match c {
			'A' | 'X' => Self::Rock,
			'B' | 'Y' => Self::Paper,
			'C' | 'Z' => Self::Scissors,
			_ => panic!(),
		}
	}
}
fn calculate_move(opponent: Move, outcome: Outcome) -> Move {
	match opponent {
		Move::Rock => match outcome {
			Outcome::Win => Move::Paper,
			Outcome::Draw => Move::Rock,
			Outcome::Lose => Move::Scissors,
		},
		Move::Paper => match outcome {
			Outcome::Win => Move::Scissors,
			Outcome::Draw => Move::Paper,
			Outcome::Lose => Move::Rock,
		},
		Move::Scissors => match outcome {
			Outcome::Win => Move::Rock,
			Outcome::Draw => Move::Scissors,
			Outcome::Lose => Move::Paper,
		},
	}
}

fn score(you: Move, opponent: Move) -> u8 {
	you.value() + you.battle(opponent).value()
}
