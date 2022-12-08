#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
#![allow(dead_code)]

const INPUT: &str = include_str!("../data/input.txt");
const SAMPLE: &str = include_str!("../data/sample.txt");

use logos::{Lexer, Logos};
use std::{collections::HashMap, iter::Peekable};

fn main() {
	assert_eq!(run1(SAMPLE), 95437);
	println!("Task1: {}", run1(INPUT));

	assert_eq!(run2(SAMPLE), 24_933_642);
	println!("Task2: {}", run2(INPUT));
}

fn run1(s: &str) -> usize {
	let filesystem = parse_input(s);
	let mut sum = 0;
	visit_dirs("root", &filesystem, &mut |_name, dir| {
		if dir.size() < 100_000 {
			sum += dir.size();
		}
	});
	sum
}

fn run2(s: &str) -> usize {
	const DISK_SIZE: usize = 70_000_000;
	const REQUIRED_SIZE: usize = 30_000_000;
	const TARGET_SIZE: usize = DISK_SIZE - REQUIRED_SIZE;

	let filesystem = parse_input(s);
	let root_size = filesystem.size();

	let diff = root_size - TARGET_SIZE;
	let mut options = Vec::new();
	visit_dirs("root", &filesystem, &mut |_name, dir| {
		if dir.size() > diff {
			options.push(dir.size());
		}
	});
	options.sort_unstable();
	*options.first().unwrap()
}

#[derive(Default, Debug)]
struct Directory<'a> {
	directories: HashMap<&'a str, Directory<'a>>,
	files: Vec<(&'a str, usize)>,
}

impl<'a> Directory<'a> {
	pub fn size(&self) -> usize {
		self.files.iter().map(|(_, size)| *size).sum::<usize>()
			+ self
				.directories
				.values()
				.map(Directory::size)
				.sum::<usize>()
	}
}

fn visit_dirs<F>(name: &str, dir: &Directory, f: &mut F)
where
	F: FnMut(&str, &Directory),
{
	f(name, dir);
	for (name, child) in &dir.directories {
		visit_dirs(name, child, f);
	}
}

type Lex<'a> = Peekable<Lexer<'a, Token<'a>>>;

#[derive(Default, Debug)]
struct State<'a> {
	path_stack: Vec<&'a str>,
	filesystem: Directory<'a>,
}

impl<'a> State<'a> {
	fn get_current_dir(&mut self) -> &mut Directory<'a> {
		let mut cur_dir = &mut self.filesystem;
		for name in &self.path_stack {
			let entry = cur_dir.directories.entry(name).or_default();
			cur_dir = entry;
		}
		cur_dir
	}
}

fn parse_input(s: &str) -> Directory {
	let mut state = State::default();
	let mut lex = Token::lexer(s).peekable();
	while lex.peek().is_some() {
		expect_token(&mut lex, Token::Command);
		parse_command(&mut lex, &mut state);
	}
	state.filesystem
}

fn parse_command<'a>(lex: &mut Lex<'a>, state: &mut State<'a>) {
	log::trace!("parse_command");
	match lex.next().unwrap() {
		Token::ChangeDir => parse_change_dir(lex, state),
		Token::List => parse_list(lex, state),
		token => panic!("Unexpected token: {token:?}"),
	}
}

fn parse_change_dir<'a>(lex: &mut Lex<'a>, state: &mut State<'a>) {
	log::trace!("parse_change_dir");
	match lex.next().unwrap() {
		Token::Root => {
			log::trace!("/");
			state.path_stack.clear();
		}
		Token::Filename(name) => {
			log::trace!("push {name}");
			state.path_stack.push(name);
		}
		Token::GoUp => {
			log::trace!("pop");
			state.path_stack.pop();
		}
		token => panic!("Unexpected token: {token:?}"),
	}
}

fn parse_list<'a>(lex: &mut Lex<'a>, state: &mut State<'a>) {
	log::trace!("parse_list");
	while lex.peek().is_some() && !matches!(lex.peek(), Some(Token::Command)) {
		match lex.next().unwrap() {
			Token::Dir => {
				let name = expect_filename(lex);
				log::trace!("Dir listed: {name}");
				let dir = state.get_current_dir();
				dir.directories.entry(name).or_default();
			}
			Token::Size(size) => {
				let name = expect_filename(lex);
				log::trace!("File listed: {name} : {size}");
				let dir = state.get_current_dir();
				dir.files.push((name, size));
			}
			token => panic!("Unexpected token: {token:?}"),
		}
	}
}

fn expect_filename<'a>(lex: &mut Lex<'a>) -> &'a str {
	match lex.next().unwrap() {
		Token::Filename(name) => name,
		token => panic!("Unexpected token: {token:?}"),
	}
}

fn expect_size(lex: &mut Lex) -> usize {
	match lex.next().unwrap() {
		Token::Size(size) => size,
		token => panic!("Unexpected token: {token:?}"),
	}
}

fn expect_token(lex: &mut Lex, t: Token) {
	assert!(lex.next().unwrap() == t);
}

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
enum Token<'a> {
	#[regex(r"[\d]+", |lex| lex.slice().parse())]
	Size(usize),

	#[regex(r"[\w]*[\.]?[\w]*")]
	Filename(&'a str),

	#[token("$")]
	Command,

	#[token("cd")]
	ChangeDir,

	#[token("ls")]
	List,

	#[token("/")]
	Root,

	#[token("dir")]
	Dir,

	#[token("..")]
	GoUp,

	#[error]
	#[regex(r"[ \t\r\n\f]+", logos::skip)]
	Error,
}
