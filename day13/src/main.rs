#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::wildcard_imports)]
#![allow(dead_code)]

const INPUT: &str = include_str!("../data/input.txt");
const SAMPLE: &str = include_str!("../data/sample.txt");

use logos::{Lexer, Logos};
use std::iter::Peekable;

fn main() {
	let sample = run(SAMPLE);
	println!("1. Sample: {sample}");
	assert_eq!(sample, 13);

	let input = run(INPUT);
	println!("1. Input: {input}");

	let sample = run2(SAMPLE);
	println!("2. Sample: {sample}");
	assert_eq!(sample, 140);

	let input = run2(INPUT);
	println!("2. Input: {input}");
}

#[test]
fn test() {
	assert_eq!(run(SAMPLE), 13);
	assert_eq!(run(INPUT), 5843);
	assert_eq!(run2(SAMPLE), 140);
	assert_eq!(run2(INPUT), 26289);
}

fn run(s: &str) -> usize {
	let mut packets = s.lines().filter(|line| !line.is_empty()).map(Packet::from);
	let mut index = 0;
	let mut sum = 0;
	while let Some(left) = packets.next() {
		if let Some(right) = packets.next() {
			index += 1;
			// println!("[{index}]\n{left:?}\nvs\n{right:?}");
			if let Some(ord) = left.partial_cmp(&right) {
				if ord.is_lt() {
					sum += index;
				}
				// println!("{ord:?}");
			}
			// println!();
		}
	}
	sum
}

fn run2(s: &str) -> usize {
	let mut packets = s
		.lines()
		.filter(|line| !line.is_empty())
		.map(Packet::from)
		.collect::<Vec<_>>();
	packets.push(Packet::List(vec![Packet::Value(2)]));
	packets.push(Packet::List(vec![Packet::Value(6)]));
	packets.sort();

	// println!("Sorted");
	// for (index, packet) in packets.iter().enumerate() {
	// 	println!("[{}] {:?}", index + 1, packet);
	// }
	// println!();

	let mut product = 1;
	for (index, packet) in packets.iter().enumerate() {
		if *packet == Packet::List(vec![Packet::Value(2)])
			|| *packet == Packet::List(vec![Packet::Value(6)])
		{
			product *= index + 1;
		}
	}

	product
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet {
	List(Vec<Packet>),
	Value(i32),
}

impl Packet {
	pub fn as_list(&self) -> Vec<Self> {
		match self {
			Self::List(v) => v.clone(),
			Self::Value(i) => vec![Self::Value(*i)],
		}
	}
}

impl PartialOrd for Packet {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		// println!("{self:?} vs {other:?}");

		if let Self::Value(left) = self {
			if let Self::Value(right) = other {
				if left == right {
					return None;
				}
				return left.partial_cmp(right);
			}
		}

		if let Self::List(left) = self {
			if let Self::List(right) = other {
				for (left, right) in left.iter().zip(right.iter()) {
					if let Some(ord) = left.partial_cmp(right) {
						return Some(ord);
					}
				}

				if left.len() != right.len() {
					return left.len().partial_cmp(&right.len());
				}
				return None;
			}
		}

		let left = self.as_list();
		let right = other.as_list();
		for (left, right) in left.iter().zip(right.iter()) {
			if let Some(ord) = left.partial_cmp(right) {
				return Some(ord);
			}
		}

		if left.len() != right.len() {
			return left.len().partial_cmp(&right.len());
		}

		None
	}
}

impl Ord for Packet {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.partial_cmp(other).unwrap()
	}
}

type Lex<'a> = Peekable<Lexer<'a, Token>>;

fn parse_list(lex: &mut Lex) -> Packet {
	assert_eq!(lex.next().unwrap(), Token::ListOpen);
	let mut out = Vec::new();
	loop {
		match lex.peek() {
			Some(Token::ListOpen) => {
				let packet = parse_list(lex);
				out.push(packet);
			}
			Some(Token::ListClose) => {
				lex.next().unwrap();
				break;
			}
			Some(Token::Integer(i)) => {
				out.push(Packet::Value(*i));
				lex.next().unwrap();
			}
			Some(Token::Separator | Token::Error) => {
				lex.next().unwrap();
			}
			None => panic!(),
		}
	}

	Packet::List(out)
}

impl From<&str> for Packet {
	fn from(s: &str) -> Self {
		let mut lex = Lexer::<Token>::new(s).peekable();
		parse_list(&mut lex)
	}
}

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
enum Token {
	#[regex(r"[\d]+", |lex| lex.slice().parse())]
	Integer(i32),

	#[token("[")]
	ListOpen,

	#[token("]")]
	ListClose,

	#[token(",")]
	Separator,

	#[error]
	#[regex(r"[ \t\r\n\f:]+", logos::skip)]
	Error,
}
