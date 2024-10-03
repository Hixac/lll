
#[derive(Debug)]
pub enum Literal {
	Float(f64),
	String(String),
	Bool(bool),
	Identifier(String),
	Nil
}

impl Literal { // sum, sub, mul, div
	pub fn sum(v1: Literal, v2: Literal) -> Literal {
		use Literal::*;
		match (v1, v2) {
			(String(v1), String(v2)) => { String(format!("{v1}{v2}")) },
			(Float(v1), Float(v2)) => { Float(v1 + v2) },
			(String(_), Float(_)) => {
				eprintln!("FATAL: cannot sum string and float");
				std::process::exit(69);
			},
			(Float(_), String(_)) => {
				eprintln!("FATAL: cannot sum float and string");
				std::process::exit(69);
			},
			(Bool(v1), Bool(v2)) => { Bool(v1 || v2) }
			_ => {
				eprintln!("FATAL: cannot sum nil or identifier");
				std::process::exit(69);
			}
		}
	}

	pub fn sub(v1: Literal, v2: Literal) -> Literal {
		use Literal::*;
		match (v1, v2) {
			(String(_), String(_)) => {
				eprintln!("FATAL: cannot sub string and string");
				std::process::exit(69);
			},
			(Float(v1), Float(v2)) => { Float(v1 - v2) },
			(String(_), Float(_)) => {
				eprintln!("FATAL: cannot sub string and float");
				std::process::exit(69);
			},
			(Float(_), String(_)) => {
				eprintln!("FATAL: cannot sub float and string");
				std::process::exit(69);
			},
			(Bool(_), Bool(_)) => {
				eprintln!("FATAL: cannot sub bool and bool");
				std::process::exit(69);
			}
			_ => {
				eprintln!("FATAL: cannot sub nil or identifier");
				std::process::exit(69);
			}
		}
	}

	pub fn mul(v1: Literal, v2: Literal) -> Literal {
		use Literal::*;
		match (v1, v2) {
			(String(_), String(_)) => {
				eprintln!("FATAL: cannot mul string and string");
				std::process::exit(69);
			},
			(Float(v1), Float(v2)) => { Float(v1 * v2) },
			(String(_), Float(_)) => {
				eprintln!("FATAL: cannot mul string and float");
				std::process::exit(69);
			},
			(Float(_), String(_)) => {
				eprintln!("FATAL: cannot mul float and string");
				std::process::exit(69);
			},
			(Bool(v1), Bool(v2)) => { Bool(v1 && v2) }
			_ => {
				eprintln!("FATAL: cannot mul nil or identifier");
				std::process::exit(69);
			}
		}
	}

	pub fn div(v1: Literal, v2: Literal) -> Literal {
		use Literal::*;
		match (v1, v2) {
			(String(_), String(_)) => {
				eprintln!("FATAL: cannot div string and string");
				std::process::exit(69);
			},
			(Float(v1), Float(v2)) => { Float(v1 / v2) },
			(String(_), Float(_)) => {
				eprintln!("FATAL: cannot div string and float");
				std::process::exit(69);
			},
			(Float(_), String(_)) => {
				eprintln!("FATAL: cannot div float and string");
				std::process::exit(69);
			},
			(Bool(_), Bool(_)) => {
				eprintln!("FATAL: cannot div bool and bool");
				std::process::exit(69);
			}
			_ => {
				eprintln!("FATAL: cannot div nil or identifier");
				std::process::exit(69);
			}
		}
	}
}

impl ToString for Literal {
	fn to_string(&self) -> String {
		use Literal::*;
		match self {
			Float(v) => v.to_string(),
			String(v) => v.clone(),
			Bool(v) => {
				if *v {
					format!("true")
				} else {
					format!("false")
				}
			},
			Identifier(v) => v.clone(),
			Nil => format!("nil")
		}
	}
}

impl Clone for Literal {
	fn clone(&self) -> Self {
		use Literal::*;
		match self {
			Float(v) => Float(v.clone()),
			String(v) => String(v.clone()),
			Bool(v) => Bool(v.clone()),
			Identifier(v) => Identifier(v.clone()),
			Nil => Nil
		}
	}
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TokenType {
		// Single-character tokens.
		LeftParen, RightParen, LeftBrace, RightBrace,
		Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

		// One or two character tokens.
		Bang, BangEqual,
		Equal, EqualEqual,
		Greater, GreaterEqual,
		Less, LessEqual,

		// Literals.
		Identifier, String, Number,

		// Keywords.
		And, Class, Else, False, Fun, For, If, Nil, Or,
		Print, Return, Super, This, True, Var, While,

		Eof
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct Token {
	pub literal: Literal,
	pub toktype: TokenType,
	pub place: usize,
	pub line: usize
}

impl ToString for Token {
	fn to_string(&self) -> String {
		format!("[INFO] TOKEN( literal: {}, toktype: {}, place: {}, line: {} )", self.literal.to_string(), self.toktype.to_string(), self.place, self.line)
	}
}
