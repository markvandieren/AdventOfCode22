use crate::vec2::Vec2;
use logos::{Lexer, Logos};
use std::iter::Peekable;

type Lex<'a> = Peekable<Lexer<'a, Token>>;

pub fn read_positions(s: &str) -> Vec<(Vec2, Vec2)> {
	let mut lex = Lexer::<Token>::new(s).peekable();
	let mut out = Vec::new();
	loop {
		if lex.peek().is_none() {
			break;
		}
		consume_one(&mut lex); // sensor
		consume_one(&mut lex); // x
		let sensor_x = expect_integer(&mut lex);
		consume_one(&mut lex); // ,
		consume_one(&mut lex); // y
		let sensor_y = expect_integer(&mut lex);
		consume_one(&mut lex); // beacon
		consume_one(&mut lex); // x
		let beacon_x = expect_integer(&mut lex);
		consume_one(&mut lex); // ,
		consume_one(&mut lex); // y
		let beacon_y = expect_integer(&mut lex);
		out.push((Vec2::new(sensor_x, sensor_y), Vec2::new(beacon_x, beacon_y)));
	}
	out
}

fn consume_one(lex: &mut Lex) -> Token {
	lex.next().unwrap()
}

fn expect_integer(lex: &mut Lex) -> i32 {
	let t = lex.next().unwrap();
	if let Token::Integer(i) = t {
		i
	} else {
		panic!("Expected integer, got: {t:?}")
	}
}

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
enum Token {
	#[regex(r"[-]?[\d]+", |lex| lex.slice().parse())]
	Integer(i32),

	#[token("Sensor at")]
	Sensor,

	#[token(": closest beacon is at")]
	Beacon,

	#[token("x=")]
	X,

	#[token("y=")]
	Y,

	#[token(",")]
	Comma,

	#[error]
	#[regex(r"[ \t\r\n\f:]+", logos::skip)]
	Error,
}
