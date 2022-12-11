use crate::common::*;
use logos::{Lexer, Logos};
use std::iter::Peekable;

pub fn monkeys(s: &str) -> Vec<Monkey> {
	let mut monkeys = Vec::new();
	let mut lex = Lexer::<Token>::new(s).peekable();
	while lex.peek().is_some() {
		expect_token(&mut lex, Token::Monkey);
		expect_token(&mut lex, Token::StartingItems);

		let mut items = Vec::new();
		while matches!(lex.peek(), Some(Token::Integer(_))) {
			items.push(expect_int(&mut lex));
			consume_if(&mut lex, Token::Comma);
		}

		let op = parse_operation(&mut lex);

		expect_token(&mut lex, Token::Test);
		let test = expect_int(&mut lex);

		expect_token(&mut lex, Token::IfTrue);
		let if_true = usize::try_from(expect_int(&mut lex)).unwrap();
		expect_token(&mut lex, Token::IfFalse);
		let if_false = usize::try_from(expect_int(&mut lex)).unwrap();

		monkeys.push(Monkey {
			items,
			op,
			test,
			if_true,
			if_false,
			inspection_count: 0,
		});
	}
	monkeys
}

fn parse_operation(lex: &mut Lex) -> Operation {
	expect_token(lex, Token::Operation);

	if consume_if(lex, Token::Add) {
		Operation::Add(parse_operation_value(lex))
	} else if consume_if(lex, Token::Mul) {
		Operation::Mul(parse_operation_value(lex))
	} else {
		panic!()
	}
}

fn parse_operation_value(lex: &mut Lex) -> Value {
	let token = lex.next().unwrap();
	if let Token::Integer(num) = token {
		Value::Num(num)
	} else if let Token::Old = token {
		Value::Old
	} else {
		panic!()
	}
}

type Lex<'a> = Peekable<Lexer<'a, Token>>;

fn expect_token(lex: &mut Lex, t: Token) {
	let next = lex.next().unwrap();
	assert_eq!(next, t);
}

fn consume_if(lex: &mut Lex, t: Token) -> bool {
	if lex.peek() == Some(&t) {
		lex.next().unwrap();
		true
	} else {
		false
	}
}

fn expect_int(lex: &mut Lex) -> u128 {
	match lex.next().unwrap() {
		Token::Integer(i) => i,
		token => panic!("Unexpected token: {token:?}"),
	}
}

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
enum Token {
	#[regex(r"[\d]+", |lex| lex.slice().parse())]
	Integer(u128),

	#[regex(r"Monkey [\d]:")]
	Monkey,

	#[token("Starting items:")]
	StartingItems,

	#[token("Operation: new = old")]
	Operation,

	#[token("Test: divisible by")]
	Test,

	#[token("If true: throw to monkey")]
	IfTrue,

	#[token("If false: throw to monkey")]
	IfFalse,

	#[token(",")]
	Comma,

	#[token("+")]
	Add,

	#[token("old")]
	Old,

	#[token("*")]
	Mul,

	#[token("=")]
	Assign,

	#[error]
	#[regex(r"[ \t\r\n\f:]+", logos::skip)]
	Error,
}
