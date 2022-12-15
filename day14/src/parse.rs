use crate::pos::Pos;
use logos::{Lexer, Logos};
use std::iter::Peekable;

type Lex<'a> = Peekable<Lexer<'a, Token>>;

pub fn path(s: &str) -> Vec<Pos> {
	let mut lex = Lexer::<Token>::new(s).peekable();
	let mut path = Vec::new();
	loop {
		if lex.peek().is_none() {
			break;
		}
		let coord = parse_coordinate(&mut lex);
		path.push(coord);

		if let Some(Token::Arrow) = lex.peek() {
			lex.next().unwrap();
		}
	}
	path
}

fn parse_coordinate(lex: &mut Lex) -> Pos {
	let x = expect_integer(lex);
	consume_one(lex);
	let y = expect_integer(lex);
	Pos::new(x, y)
}

fn consume_one(lex: &mut Lex) -> Token {
	lex.next().unwrap()
}

fn expect_integer(lex: &mut Lex) -> i32 {
	if let Token::Integer(i) = lex.next().unwrap() {
		i
	} else {
		panic!()
	}
}

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
enum Token {
	#[regex(r"[\d]+", |lex| lex.slice().parse())]
	Integer(i32),

	#[token(",")]
	Comma,

	#[token("->")]
	Arrow,

	#[error]
	#[regex(r"[ \t\r\n\f:]+", logos::skip)]
	Error,
}
