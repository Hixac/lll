
use super::error::Error;

#[derive(Debug)]
pub enum Literal {
	Float(f64),
	String(String),
	Bool(bool),
	Identifier(String),
	Nil
}

impl Literal { // sum, sub, mul, div, cmp
	pub fn sum(v1: Literal, v2: Literal) -> Result<Literal, Error> {
		use Literal::*;
		match (v1, v2) {
			(String(v1), String(v2)) => Ok(String(format!("{v1}{v2}"))),
			(Float(v1), Float(v2)) => Ok(Float(v1 + v2)),
			(String(_), Float(_)) => Err(Error::fatal("FATAL: cannot sum string and float", None)),
			(Float(_), String(_)) => Err(Error::fatal("FATAL: cannot sum float and string", None)),
			(Bool(v1), Bool(v2)) => Ok(Bool(v1 || v2)),
			_ => Err(Error::fatal("FATAL: cannot sum nil or identifier", None))
		}
	}

	pub fn sub(v1: Literal, v2: Literal) -> Result<Literal, Error> {
		use Literal::*;
		match (v1, v2) {
			(String(_), String(_)) => Err(Error::fatal("FATAL: cannot sub string and string", None)),
			(Float(v1), Float(v2)) => Ok(Float(v1 - v2)),
			(String(_), Float(_)) => Err(Error::fatal("FATAL: cannot sub string and float", None)),
			(Float(_), String(_)) => Err(Error::fatal("FATAL: cannot sub float and string", None)),
			(Bool(_), Bool(_)) => Err(Error::fatal("FATAL: cannot sub bool and bool", None)),
			_ => Err(Error::fatal("FATAL: cannot sub nil or identifier", None)),
		}
	}

	pub fn mul(v1: Literal, v2: Literal) -> Result<Literal, Error> {
		use Literal::*;
		match (v1, v2) {
			(String(_), String(_)) => Err(Error::fatal("FATAL: cannot mul string and string", None)),
			(Float(v1), Float(v2)) => Ok(Float(v1 * v2)),
			(String(_), Float(_)) => Err(Error::fatal("FATAL: cannot mul string and float", None)),
			(Float(_), String(_)) => Err(Error::fatal("FATAL: cannot mul float and string", None)),
			(Bool(v1), Bool(v2)) => Ok(Bool(v1 && v2)),
			_ => Err(Error::fatal("FATAL: cannot mul nil or identifier", None)),
		}
	}

	pub fn div(v1: Literal, v2: Literal) -> Result<Literal, Error> {
		use Literal::*;
		match (v1, v2) {
			(String(_), String(_)) => Err(Error::fatal("FATAL: cannot div string and string", None)),
			(Float(v1), Float(v2)) => Ok(Float(v1 / v2)),
			(String(_), Float(_)) => Err(Error::fatal("FATAL: cannot div string and float", None)),
			(Float(_), String(_)) => Err(Error::fatal("FATAL: cannot div float and string", None)),
			(Bool(_), Bool(_)) => Err(Error::fatal("FATAL: cannot div bool and bool", None)),
			_ => Err(Error::fatal("FATAL: cannot div nil or identifier", None)),
		}
	}

	pub fn eq(v1: Literal, v2: Literal) -> Result<Literal, Error> {
		use Literal::*;
		match (v1, v2) {
			(String(v1), String(v2)) => Ok(Bool(v1 == v2)),
			(Float(v1), Float(v2)) => Ok(Bool(v1 == v2) ),
			(String(_), Float(_)) => Err(Error::fatal("FATAL: cannot eq string and float", None)),
			(Float(_), String(_)) => Err(Error::fatal("FATAL: cannot eq float and string", None)),
			(Bool(v1), Bool(v2)) => Ok(Bool(v1 == v2)),
			_ => Err(Error::fatal("FATAL: cannot eq nil or identifier", None)),
		}
	}

	pub fn gt(v1: Literal, v2: Literal) -> Result<Literal, Error> {
		use Literal::*;
		match (v1, v2) {
			(String(v1), String(v2)) => Ok(Bool(v1.len() > v2.len())),
			(Float(v1), Float(v2)) => Ok(Bool(v1 > v2)),
			(String(_), Float(_)) => Err(Error::fatal("FATAL: cannot gt string and float", None)),
			(Float(_), String(_)) => Err(Error::fatal("FATAL: cannot gt float and string", None)),
			(Bool(v1), Bool(v2)) => Ok(Bool(v1 > v2)),
			_ => Err(Error::fatal("FATAL: cannot gt nil or identifier", None)),
		}
	}

	pub fn egt(v1: Literal, v2: Literal) -> Result<Literal, Error> {
		use Literal::*;
		match (v1, v2) {
			(String(v1), String(v2)) => Ok(Bool(v1.len() >= v2.len())),
			(Float(v1), Float(v2)) => Ok(Bool(v1 >= v2)),
			(String(_), Float(_)) => Err(Error::fatal("FATAL: cannot gt string and float", None)),
			(Float(_), String(_)) => Err(Error::fatal("FATAL: cannot gt float and string", None)),
			(Bool(v1), Bool(v2)) => Ok(Bool(v1 >= v2)),
			_ => Err(Error::fatal("FATAL: cannot gt nil or identifier", None)),
		}
	}

	pub fn lt(v1: Literal, v2: Literal) -> Result<Literal, Error> {
		use Literal::*;
		match (v1, v2) {
			(String(v1), String(v2)) => Ok(Bool(v1.len() < v2.len())),
			(Float(v1), Float(v2)) => Ok(Bool(v1 < v2)),
			(String(_), Float(_)) => Err(Error::fatal("FATAL: cannot gt string and float", None)),
			(Float(_), String(_)) => Err(Error::fatal("FATAL: cannot gt float and string", None)),
			(Bool(v1), Bool(v2)) => Ok(Bool(v1 < v2)),
			_ => Err(Error::fatal("FATAL: cannot gt nil or identifier", None)),
		}
	}

	pub fn elt(v1: Literal, v2: Literal) -> Result<Literal, Error> {
		use Literal::*;
		match (v1, v2) {
			(String(v1), String(v2)) => Ok(Bool(v1.len() <= v2.len())),
			(Float(v1), Float(v2)) => Ok(Bool(v1 <= v2)),
			(String(_), Float(_)) => Err(Error::fatal("FATAL: cannot gt string and float", None)),
			(Float(_), String(_)) => Err(Error::fatal("FATAL: cannot gt float and string", None)),
			(Bool(v1), Bool(v2)) => Ok(Bool(v1 <= v2)),
			_ => Err(Error::fatal("FATAL: cannot gt nil or identifier", None))
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
		Print, Return, Super, This, True, New, While,

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
