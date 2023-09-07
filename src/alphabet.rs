use std::{collections::HashMap, sync::OnceLock};

static ALPHABET: OnceLock<HashMap<char, String>> = OnceLock::new();

fn init_alphabet() -> HashMap<char, String> {
	let mut hm = HashMap::<char, String>::new();
	hm.insert('a', "alpha".to_string());
	hm.insert('b', "bravo".to_string());
	hm.insert('c', "charlie".to_string());
	hm.insert('d', "delta".to_string());
	hm.insert('e', "echo".to_string());
	hm.insert('f', "fox".to_string());
	hm.insert('g', "golf".to_string());
	hm.insert('h', "hotel".to_string());
	hm.insert('i', "india".to_string());
	hm.insert('j', "juliet".to_string());
	hm.insert('k', "kilo".to_string());
	hm.insert('l', "lima".to_string());
	hm.insert('m', "mike".to_string());
	hm.insert('n', "november".to_string());
	hm.insert('o', "oscar".to_string());
	hm.insert('p', "papa".to_string());
	hm.insert('q', "quebec".to_string());
	hm.insert('r', "romeo".to_string());
	hm.insert('s', "sierra".to_string());
	hm.insert('t', "tango".to_string());
	hm.insert('u', "uniform".to_string());
	hm.insert('v', "victor".to_string());
	hm.insert('w', "whiskey".to_string());
	hm.insert('x', "x-ray".to_string());
	hm.insert('y', "yankee".to_string());
	hm.insert('z', "zulu".to_string());
	hm.insert('0', "zero".to_string());
	hm.insert('1', "one".to_string());
	hm.insert('2', "two".to_string());
	hm.insert('3', "three".to_string());
	hm.insert('4', "four".to_string());
	hm.insert('5', "five".to_string());
	hm.insert('6', "six".to_string());
	hm.insert('7', "seven".to_string());
	hm.insert('8', "eight".to_string());
	hm.insert('9', "nine".to_string());
	hm.insert('@', "at".to_string());
	hm.insert(' ', "space".to_string());
	hm.insert('.', "dot".to_string());
	hm.insert(':', "colon".to_string());
	hm.insert(';', "semi-colon".to_string());
	hm.insert('!', "exclamation mark".to_string());
	hm.insert('?', "question mark".to_string());
	hm.insert('\'', "single quote".to_string());
	hm.insert('"', "double quote".to_string());
	hm.insert('(', "left parenthesis".to_string());
	hm.insert(')', "right parenthesis".to_string());
	hm.insert('-', "dash".to_string());
	hm.insert('_', "underscore".to_string());
	hm.insert('/', "slash".to_string());
	hm.insert('\\', "backslash".to_string());
	hm.insert('=', "equals".to_string());
	hm.insert('+', "plus".to_string());
	hm.insert('-', "minus".to_string());
	hm.insert('#', "hash".to_string());
	hm.insert('*', "asterisk".to_string());
	hm.insert('~', "tilde".to_string());
	hm.insert('`', "backtick".to_string());
	hm.insert('<', "less than".to_string());
	hm.insert('>', "greater than".to_string());
	hm.insert('|', "pipe".to_string());
	hm
}

pub fn alphabet() -> HashMap<char, String> {
	ALPHABET.get_or_init(init_alphabet).clone()
}

pub fn to_alphabet(s: String) -> Vec<String> {
	let mut ret = Vec::<String>::new();
	let alphabet = alphabet();
	for c in s.to_lowercase().chars() {
		let cstr = c.to_string();
		let w = alphabet.get(&c).unwrap_or(&cstr);
		ret.push(format!("{} -> {}", c, w));
	}
	ret
}
