
use super::token::Token;
use super::token::TokenType;
use super::token::Literal;
use super::error::Error;

pub trait Expr: Eval { }
pub trait Eval {
	fn eval(&mut self) -> Literal;
}

struct Constant {
	v: Literal,
}

impl Expr for Constant {}
impl Eval for Constant {
	fn eval(&mut self) -> Literal {
		self.v.clone()
	}
}

struct Group {
	v: Box<dyn Expr>,
}

impl Expr for Group {}
impl Eval for Group {
	fn eval(&mut self) -> Literal {
		self.v.eval()
	}
}

struct Unary {
	v: Box<dyn Expr>,
	tok: Token
}

impl Expr for Unary {}
impl Eval for Unary {
	fn eval(&mut self) -> Literal {
		use TokenType::*;
		match self.tok.toktype {
			Minus => { return Literal::sub(Literal::Float(0.0), self.v.eval()) },
			Bang => {
				match self.v.eval() {
					Literal::Bool(v) => { return Literal::Bool(!v) }
					_ => {
						eprintln!("FATAL: unexpected literal");
						std::process::exit(69);
					}
				}
			},
			_ => {
				eprintln!("FATAL: unexpected unary token");
				std::process::exit(69);
			}
		}
	}
}

struct Binary {
	v1: Box<dyn Expr>,
	tok: Token,
	v2: Box<dyn Expr>,
}

impl Expr for Binary {}
impl Eval for Binary {
	fn eval(&mut self) -> Literal {
		use TokenType::*;
		match self.tok.toktype {
			Plus => { return Literal::sum(self.v1.eval(), self.v2.eval()) }
			Minus => { return Literal::sub(self.v1.eval(), self.v2.eval()) }
			Star => { return Literal::mul(self.v1.eval(), self.v2.eval()) }
			Slash => { return Literal::div(self.v1.eval(), self.v2.eval()) }
			_ => {
				eprintln!("FATAL: wrong operation");
				std::process::exit(69)
			}
		}
	}
}

pub struct Parser {
	tokens: Vec<Token>,
	current: usize,
}

type ResExpr = Result<Box<dyn Expr>, Error>;
impl Parser {
	pub fn new(tokens: Vec<Token>) -> Self {
		Self { tokens, current: 0 }
	}

	pub fn parse(&mut self) -> Option<Box<dyn Expr>> {
		match self.expression() {
			Ok(v) => return Some(v),
			Err(v) => {
				println!("{v}");
				return None
			}
		}
	}

	fn is_at_end(&self) -> bool {
		self.tokens[self.current].toktype == TokenType::Eof
	}

	fn synchronize(&mut self) {
		use TokenType::*;
		
		if !self.is_at_end() { self.current += 1 }

		while !self.is_at_end() {
			if self.tokens[self.current - 1].toktype == Semicolon {
				return;
			}

			match self.tokens[self.current].toktype {
				Class | Fun | Var | For | If | While | Print | Return => return,
				_ => ()
			}

			if !self.is_at_end() { self.current += 1 }
		}
	}
	
	fn expression(&mut self) -> ResExpr {
		self.equality()
	}

	fn equality(&mut self) -> ResExpr {
		let mut expr1 = self.comparison();

		while self.select(&[TokenType::BangEqual, TokenType::EqualEqual]) {
			let op = self.tokens[self.current - 1].clone();
			let expr2 = self.comparison();
			expr1 = Ok(Box::new(Binary { v1: expr1?, tok: op, v2: expr2? }));
		}

		expr1
	}

	fn comparison(&mut self) -> ResExpr {
		let mut expr1 = self.term();

		while self.select(&[TokenType::Less, TokenType::LessEqual,
							TokenType::Greater, TokenType::GreaterEqual]) {
			let op = self.tokens[self.current - 1].clone();
			let expr2 = self.term();
			expr1 = Ok(Box::new(Binary { v1: expr1?, tok: op, v2: expr2? }));
		}

		expr1
	}

	fn term(&mut self) -> ResExpr {
		let mut expr1 = self.factor();

		while self.select(&[TokenType::Minus, TokenType::Plus]) {
			let op = self.tokens[self.current - 1].clone();
			let expr2 = self.factor();
			expr1 = Ok(Box::new(Binary { v1: expr1?, tok: op, v2: expr2? }));
		}

		expr1
	}

	fn factor(&mut self) -> ResExpr {
		let mut expr1 = self.unary();

		while self.select(&[TokenType::Slash, TokenType::Star]) {
			let op = self.tokens[self.current - 1].clone();
			let expr2 = self.unary();
			expr1 = Ok(Box::new(Binary { v1: expr1?, tok: op, v2: expr2? }));
		}

		expr1
	}

	fn unary(&mut self) -> ResExpr {
		if self.select(&[TokenType::Minus, TokenType::Bang]) {
			let op = self.tokens[self.current - 1].clone();
			let expr = self.primary();
			return Ok(Box::new(Unary { v: expr?, tok: op }))
		}

		self.primary()
	}

	fn primary(&mut self) -> ResExpr {
		if self.select(&[TokenType::True, TokenType::False, TokenType::Nil, TokenType::Number, TokenType::String]) {
			return Ok(Box::new(Constant { v: self.tokens[self.current - 1].literal.clone() }))
		} else if self.select(&[TokenType::LeftParen]) {
			let expr = self.expression();
			self.consume(TokenType::RightParen)?;
			return Ok(Box::new(Group { v: expr? }));
		}

		Err(Error::new("expected expression", Some(&self.tokens[self.current])))
	}

	fn consume(&mut self, toktype: TokenType) -> Result<(), Error> {
		if !self.is_at_end() && self.tokens[self.current].toktype == toktype {
			self.current += 1;
			return Ok(());
		}

		Err(Error::new("expected {toktype}", None))
	}

	fn select(&mut self, toktypes: &[TokenType]) -> bool {
		for i in toktypes {
			if !self.is_at_end() && self.tokens[self.current].toktype == *i {
				self.current += 1;
				return true;
			}
		}
		false
	}
}
