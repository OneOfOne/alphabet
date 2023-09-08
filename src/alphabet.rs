use std::{collections::HashMap, sync::OnceLock};

static ALPHABET: OnceLock<HashMap<char, &'static str>> = OnceLock::new();

fn init_alphabet() -> HashMap<char, &'static str> {
	let mut hm = HashMap::<char, &'static str>::new();
	hm.insert('a', "alpha");
	hm.insert('b', "bravo");
	hm.insert('c', "charlie");
	hm.insert('d', "delta");
	hm.insert('e', "echo");
	hm.insert('f', "fox");
	hm.insert('g', "golf");
	hm.insert('h', "hotel");
	hm.insert('i', "india");
	hm.insert('j', "juliet");
	hm.insert('k', "kilo");
	hm.insert('l', "lima");
	hm.insert('m', "mike");
	hm.insert('n', "november");
	hm.insert('o', "oscar");
	hm.insert('p', "papa");
	hm.insert('q', "quebec");
	hm.insert('r', "romeo");
	hm.insert('s', "sierra");
	hm.insert('t', "tango");
	hm.insert('u', "uniform");
	hm.insert('v', "victor");
	hm.insert('w', "whiskey");
	hm.insert('x', "x-ray");
	hm.insert('y', "yankee");
	hm.insert('z', "zulu");
	hm.insert('0', "zero");
	hm.insert('1', "one");
	hm.insert('2', "two");
	hm.insert('3', "three");
	hm.insert('4', "four");
	hm.insert('5', "five");
	hm.insert('6', "six");
	hm.insert('7', "seven");
	hm.insert('8', "eight");
	hm.insert('9', "nine");
	hm.insert('@', "at");
	hm.insert(' ', "space");
	hm.insert('.', "dot");
	hm.insert(':', "colon");
	hm.insert(';', "semi-colon");
	hm.insert('!', "exclamation mark");
	hm.insert('?', "question mark");
	hm.insert('\'', "single quote");
	hm.insert('"', "double quote");
	hm.insert('(', "left parenthesis");
	hm.insert(')', "right parenthesis");
	hm.insert('-', "dash");
	hm.insert('_', "underscore");
	hm.insert('/', "slash");
	hm.insert('\\', "backslash");
	hm.insert('=', "equals");
	hm.insert('+', "plus");
	hm.insert('-', "minus");
	hm.insert('#', "hash");
	hm.insert('*', "asterisk");
	hm.insert('~', "tilde");
	hm.insert('`', "backtick");
	hm.insert('<', "less than");
	hm.insert('>', "greater than");
	hm.insert('|', "pipe");
	hm
}

pub fn alphabet() -> HashMap<char, &'static str> {
	ALPHABET.get_or_init(init_alphabet).clone()
}

pub fn to_alphabet(s: String) -> Vec<(char, String)> {
	let mut ret = Vec::<(char, String)>::new();
	let alphabet = alphabet();
	for c in s.to_lowercase().chars() {
		let cstr = c.to_string();
		let w = alphabet.get(&c).unwrap_or(&cstr.as_str()).to_string();
		ret.push((c, w.to_string()));
	}
	ret
}
