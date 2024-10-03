
use crate::lll::token::TokenType;
use crate::lll::token::Literal;
use crate::lll::token::Token;
use crate::lll::error::Error;

struct Stringstream {
	text: String,
	offset: usize,
}

impl Stringstream {
	fn new(source: String) -> Self {
		Self { text: source, offset: 0 }
	}
	
	fn advance(&mut self) -> Option<char> {
		let c = self.text.chars().nth(self.offset);
		self.offset += 1;
		c
	}

	fn substring(&self, start: usize, end: usize) -> Option<String> {
		if start > self.text.len() || end > self.text.len() {
			return None
		}
		Some(self.text[start..end].to_string())
	}
	
	fn peek(&self) -> Option<char> {
		self.text.chars().nth(self.offset)
	}
}

pub struct Lexer {
	ss: Stringstream,
	line: usize,
	place: usize,
	prev_place: usize,
	
	tokens: Vec<Token>
}

impl Lexer {
	pub fn new(source: String) -> Self {
		Self { ss: Stringstream::new(source), line: 0, place: 0, prev_place: 0, tokens: vec![] }
	}

	pub fn scan_tokens(&mut self) -> Vec<Token> {
		while let Some(c) = self.advance() {
			match self.scan_token(c) {
				Ok(()) => (),
				Err(v) => println!("{v}")
			}
			self.prev_place = self.place;
		}

		self.tokens.clone()
	}

	fn scan_token(&mut self, c: char) -> Result<(), Error> {
		use TokenType::*;
		match c {
			'(' => self.add_primitive_token(LeftParen),
			')' => self.add_primitive_token(RightParen),
			'{' => self.add_primitive_token(LeftBrace),
			'}' => self.add_primitive_token(RightBrace),
			',' => self.add_primitive_token(Comma),
			'.' => self.add_primitive_token(Dot),
			'+' => self.add_primitive_token(Plus),
			'-' => self.add_primitive_token(Minus),
			'*' => self.add_primitive_token(Star),
			';' => self.add_primitive_token(Semicolon),
			'\r' | '\t' | ' ' | '\n' => {},
			'!' => {
				if self.is('=') {
					self.add_primitive_token(BangEqual);
				} else {
					self.add_primitive_token(Bang);
				}
			},
			'=' => {
				if self.is('=') {
					self.add_primitive_token(EqualEqual);
				} else {
					self.add_primitive_token(Equal);
				}
			},
			'>' => {
				if self.is('=') {
					self.add_primitive_token(GreaterEqual);
				} else {
					self.add_primitive_token(Greater);
				}
			},
			'<' => {
				if self.is('=') {
					self.add_primitive_token(LessEqual);
				} else {
					self.add_primitive_token(Less);
				}
			},
			'/' => {
				if self.is('/') {
					while let Some(c) = self.ss.peek() {
						if c == '\n' { break; }
						self.advance();
					}
				} else {
					self.add_primitive_token(Slash);
				}
			},
			'"' => self.string(),
			c if c.is_digit(10) => self.number(),
			c if c.is_alphabetic() => self.indentifier(),
			_ => {
				return Err(Error::new(format!("unexpected character, char {c}, line {}, place {}", self.line, self.place).as_str(), None));
			}
		}
		Ok(())
	}

	fn number(&mut self) {		
		let mut flag_dot = false;
		while let Some(peeker) = self.ss.peek() {
			if !flag_dot && peeker == '.' {
				flag_dot = true;
				self.advance();
				continue;
			}
			if !(peeker.is_digit(10)) {
				break;
			}

			self.advance();
		}

		let floatlit = self.substring(None, None).unwrap();
		self.add_token(Literal::Float(floatlit.parse::<f64>().unwrap()), TokenType::Number, self.prev_place, self.line);
	}
	
	fn string(&mut self) {
		let mut flag_terminated = false;
		while let Some(peeker) = self.ss.peek() {
			if peeker == '"' {
				self.advance();
				flag_terminated = true;
				break;
			}

			self.advance();
		}

		if !flag_terminated {
			eprintln!("FATAL: unterminated string at {}", self.line);
			std::process::exit(69);
		}
		
		let strlit = self.substring(None, None).unwrap();
		self.add_token(Literal::String(strlit), TokenType::String, self.prev_place, self.line);
	}
	
	fn indentifier(&mut self) {
		while let Some(peeker) = self.ss.peek() {
			if !(peeker.is_alphabetic() || peeker == '_' || peeker.is_digit(10)) {
				break
			}

			self.advance();
		}

		let iden = self.substring(None, None).unwrap();
		let Some(keyword) = self.keyword_map(&iden) else {
			self.add_token(Literal::Identifier(iden), TokenType::Identifier, self.prev_place, self.line);
			return;
		};

		if keyword == TokenType::True {
			self.add_token(Literal::Bool(true), keyword, self.prev_place, self.line);
			return
		} else if keyword == TokenType::False {
			self.add_token(Literal::Bool(false), keyword, self.prev_place, self.line);
			return
		}

		self.add_token(Literal::Nil, keyword, self.prev_place, self.line);
	}

	fn keyword_map(&self, word: &String) -> Option<TokenType> {
		use TokenType::*;
		
		if word == "and" {
			return Some(And)
		} else if word == "class" {
			return Some(Class)
		} else if word == "else" {
			return Some(Else)
		} else if word == "false" {
			return Some(False)
		} else if word == "for" {
			return Some(For)
		} else if word == "fun" {
			return Some(Fun)
		} else if word == "if" {
			return Some(If)
		} else if word == "nil" {
			return Some(Nil)
		} else if word == "or" {
			return Some(Or) 
		} else if word == "print" {
			return Some(Print)
		} else if word == "return" {
			return Some(Return)
		} else if word == "super" {
			return Some(Super)
		} else if word == "this" {
			return Some(This)
		} else if word == "true" {
			return Some(True)
		} else if word == "var" {
			return Some(Var)
		} else if word == "while" {
			return Some(While)
		}

		None
	}
	
	fn is(&mut self, expected: char) -> bool {
		let Some(c) = self.ss.peek() else {
			return false;
		};

		if c == expected {
			self.advance();
			if c == '\n' {
				self.line += 1;
			}
			return true;
		}

		return false;
	}

	fn substring(&self, start: Option<usize>, end: Option<usize>) -> Option<String> {
		let (Some(start), Some(end)) = (start, end) else {
			return self.ss.substring(self.prev_place, self.place);
		};

		self.ss.substring(start, end)
	}
	
	fn advance(&mut self) -> Option<char> {
		match self.ss.advance() {
			Some(v) => {
				self.place += 1;
				if v == '\n' { self.line += 1; }
				Some(v)
			},
			None => None
		}
	}
	
	fn add_primitive_token(&mut self, toktype: TokenType) {
		self.add_token(Literal::Nil, toktype, self.prev_place, self.line);
	}

	fn add_token(&mut self, literal: Literal, toktype: TokenType, place: usize, line: usize) {
		self.tokens.push(Token { literal, toktype, place, line });
	}
}
