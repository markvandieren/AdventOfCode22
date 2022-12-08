#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
#![allow(dead_code)]

const SAMPLE: &str = include_str!("../data/sample.txt");
const INPUT: &str = include_str!("../data/input.txt");

use logos::{Lexer, Logos};

fn main() {
	// println!("1");
	// let msg = run(SAMPLE);
	// println!("Sample message: {msg}");
	// assert_eq!(msg, "CMZ");
	// println!("Input message: {}", run(INPUT));

	println!("2");
	let msg = run2(SAMPLE);
	println!("Sample message: {msg}");
	assert_eq!(msg, "MCD");
	println!("Input message: {}", run2(INPUT));
}

fn run2(s: &str) -> String {
	let (field, moves) = s.split_once("\r\n\r\n").unwrap();
	let mut stacks = parse_field(field);
	handle_moves2(moves, &mut stacks);

	let mut message = String::new();
	for stack in stacks {
		if let Some(top_item) = stack.last() {
			message.push(*top_item);
		}
	}
	message
}

fn handle_moves2(moves: &str, stacks: &mut [Vec<char>]) {
	for instruction in moves.lines() {
		let mut lex = Token::lexer(instruction);
		lex.next().unwrap();
		let amount = expect_integer(&mut lex);
		lex.next().unwrap();
		let from = expect_integer(&mut lex) - 1;
		lex.next().unwrap();
		let to = expect_integer(&mut lex) - 1;
		println!("move {amount} from {from} to {to}");

		handle_move2(stacks, from, to, amount);
	}
}

fn run(s: &str) -> String {
	let (field, moves) = s.split_once("\r\n\r\n").unwrap();

	let mut stacks = parse_field(field);
	handle_moves(moves, &mut stacks);

	let mut message = String::new();
	for stack in stacks {
		if let Some(top_item) = stack.last() {
			message.push(*top_item);
		}
	}
	message
}

fn handle_moves(moves: &str, stacks: &mut [Vec<char>]) {
	for instruction in moves.lines() {
		let mut lex = Token::lexer(instruction);
		consume_one(&mut lex);
		let amount = expect_integer(&mut lex);
		consume_one(&mut lex);
		let from = expect_integer(&mut lex) - 1;
		consume_one(&mut lex);
		let to = expect_integer(&mut lex) - 1;
		println!("move {amount} from {from} to {to}");
		for _ in 0..amount {
			handle_move(stacks, from, to);
		}
	}
}

fn consume_one(lex: &mut Lexer<Token>) -> Token {
	lex.next().unwrap()
}

fn expect_integer(lex: &mut Lexer<Token>) -> usize {
	if let Token::Integer(i) = lex.next().unwrap() {
		i
	} else {
		panic!()
	}
}

fn handle_move(stacks: &mut [Vec<char>], from: usize, to: usize) {
	if let Some(item) = stacks.get_mut(from).unwrap().pop() {
		stacks.get_mut(to).unwrap().push(item);
	}
}

fn handle_move2(stacks: &mut [Vec<char>], from: usize, to: usize, amount: usize) {
	let mut tmp = Vec::new();
	for _ in 0..amount {
		if let Some(item) = stacks.get_mut(from).unwrap().pop() {
			tmp.push(item);
		}
	}
	for item in tmp.into_iter().rev() {
		stacks.get_mut(to).unwrap().push(item);
	}
}

#[derive(Logos, Debug, PartialEq)]
enum Token {
	#[regex(r"[\d]+", |lex| lex.slice().parse())]
	Integer(usize),

	#[token("move")]
	Move,

	#[token("from")]
	From,

	#[token("to")]
	To,

	#[error]
	#[regex(r"[ \t\n\f]+", logos::skip)]
	Error,
}

fn parse_field(s: &str) -> Vec<Vec<char>> {
	let mut iter = s.lines().rev();
	let num_stacks = parse_num_stacks(&mut iter);
	let mut stacks = vec![Vec::new(); num_stacks];
	for line in iter {
		let mut iter = line.chars().peekable();
		let mut stack_index = 0;
		while let Some(c) = iter.next() {
			if c == '[' {
				let a = consume_next(&mut iter);
				stacks[stack_index].push(a);
				consume_next(&mut iter); // Brace
				stack_index += 1;
				consume_next_if_space(&mut iter);
			} else if c == ' ' {
				stack_index += 1;
				consume_next(&mut iter);
				consume_next(&mut iter);
				consume_next_if_space(&mut iter);
			}
		}
	}

	println!("Stacks: {stacks:?}");
	stacks
}

type LineIterator<'a> = std::iter::Rev<std::str::Lines<'a>>;
type CharIterator<'a> = std::iter::Peekable<std::str::Chars<'a>>;

fn consume_next(iter: &mut CharIterator<'_>) -> char {
	iter.next().unwrap()
}

fn consume_next_if_space(iter: &mut CharIterator<'_>) {
	if let Some(c) = iter.peek() {
		if *c == ' ' {
			iter.next().unwrap();
		}
	}
}

fn parse_num_stacks(iter: &mut LineIterator<'_>) -> usize {
	iter.next()
		.unwrap()
		.trim_end()
		.rsplit(' ')
		.next()
		.unwrap()
		.parse::<usize>()
		.unwrap()
}
