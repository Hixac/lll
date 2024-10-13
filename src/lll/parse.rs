use super::token::Token;
use super::token::TokenType;
use super::token::Literal;
use super::error::Error;

pub enum Stmt { // Print, Variable, Expression
	Print(Expr),
	Variable(Token, Expr),
	Expression(Expr)
}

pub enum Expr { // Binary, Group, Unary, Variable, Constant, Assign
	Binary(Box<Expr>, Token, Box<Expr>),
	Unary(Token, Box<Expr>),
	Group(Box<Expr>),
	Variable(Token),
	Assign(Token, Box<Expr>),
	Constant(Literal)
}

pub struct Parser {
	tokens: Vec<Token>,
	current: usize,
}

type ResExpr = Result<Expr, Error>;
type ResStmt = Result<Stmt, Error>;
impl Parser {
	pub fn new(tokens: Vec<Token>) -> Self {
		Self { tokens, current: 0 }
	}

	pub fn parse(&mut self) -> Option<Vec<Stmt>> {
		
		let mut stmts: Vec<Stmt> = Vec::new();
		while !self.is_at_end() {
			match self.declaration() {
				Ok(v) => stmts.push(v),
				Err(v) => {
					println!("{v}");
					self.synchronize();
				}
			}
		}

		Some(stmts)
	}

	fn declaration(&mut self) -> ResStmt {
		if self.select(&[TokenType::New]) {
			return self.var_declaration();
		}

		self.statement()
	}

	fn var_declaration(&mut self) -> ResStmt {
		let name = self.consume(TokenType::Identifier);
		
		let mut init: ResExpr = Ok(Expr::Constant(Literal::Nil));
		if self.select(&[TokenType::Equal]) {
			init = self.expression();
		}

		self.consume(TokenType::Semicolon);
		Ok(Stmt::Variable(name?, init?))
	}
	
	fn statement(&mut self) -> ResStmt {
		if self.select(&[TokenType::Print]) {
			return self.print_statement();
		}

		self.expression_statement()
	}

	fn print_statement(&mut self) -> ResStmt {
		let expr = self.expression();
		self.consume(TokenType::Semicolon)?;

		Ok(Stmt::Print(expr?))
	}

	fn expression_statement(&mut self) -> ResStmt {
		let expr = self.expression();
		self.consume(TokenType::Semicolon)?;

		Ok(Stmt::Expression(expr?))
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
				Class | Fun | New | For | If | While | Print | Return => return,
				_ => ()
			}

			if !self.is_at_end() { self.current += 1 }
		}
	}
	
	fn expression(&mut self) -> ResExpr {
		self.assignment()
	}

	fn assignment(&mut self) -> ResExpr {
		let expr = self.equality()?;

		if self.select(&[TokenType::Equal]) {
			let equals = self.tokens[self.current - 1].clone();
			let value = self.assignment()?;

			match expr {
				Expr::Variable(t) => {
					match t.literal {
						Literal::Identifier(_) => {
							return Ok(Expr::Assign(t.clone(), Box::new(value)))
						},
						_ => return Err(Error::fatal("invalid assignment target", Some(&equals)))
					}
				},
				_ => return Err(Error::fatal("invalid assignment target", Some(&equals)))
			}
		}

		Ok(expr)
	}

	fn equality(&mut self) -> ResExpr {
		let mut expr1 = self.comparison();

		while self.select(&[TokenType::BangEqual, TokenType::EqualEqual]) {
			let op = self.tokens[self.current - 1].clone();
			let expr2 = self.comparison();
			expr1 = Ok(Expr::Binary(Box::new(expr1?), op, Box::new(expr2?)));
		}

		expr1
	}

	fn comparison(&mut self) -> ResExpr {
		let mut expr1 = self.term();

		while self.select(&[TokenType::Less, TokenType::LessEqual,
							TokenType::Greater, TokenType::GreaterEqual]) {
			let op = self.tokens[self.current - 1].clone();
			let expr2 = self.term();
			expr1 = Ok(Expr::Binary(Box::new(expr1?), op, Box::new(expr2?)));
		}

		expr1
	}

	fn term(&mut self) -> ResExpr {
		let mut expr1 = self.factor();

		while self.select(&[TokenType::Minus, TokenType::Plus]) {
			let op = self.tokens[self.current - 1].clone();
			let expr2 = self.factor();
			expr1 = Ok(Expr::Binary(Box::new(expr1?), op, Box::new(expr2?)));
		}

		expr1
	}

	fn factor(&mut self) -> ResExpr {
		let mut expr1 = self.unary();

		while self.select(&[TokenType::Slash, TokenType::Star]) {
			let op = self.tokens[self.current - 1].clone();
			let expr2 = self.unary();
			expr1 = Ok(Expr::Binary(Box::new(expr1?), op, Box::new(expr2?)));
		}

		expr1
	}

	fn unary(&mut self) -> ResExpr {
		if self.select(&[TokenType::Minus, TokenType::Bang]) {
			let op = self.tokens[self.current - 1].clone();
			let expr = self.primary();
			return Ok(Expr::Unary(op, Box::new(expr?)))
		}

		self.primary()
	}

	fn primary(&mut self) -> ResExpr {
		if self.select(&[TokenType::True, TokenType::False, TokenType::Nil, TokenType::Number, TokenType::String]) {
			return Ok(Expr::Constant(self.tokens[self.current - 1].literal.clone()))
		} else if self.select(&[TokenType::Identifier]) {
			return Ok(Expr::Variable(self.tokens[self.current - 1].clone()));
		} else if self.select(&[TokenType::LeftParen]) {
			let expr = self.expression();
			self.consume(TokenType::RightParen)?;
			return Ok(Expr::Group(Box::new(expr?)));
		}

		Err(Error::fatal("expected expression", Some(&self.tokens[self.current])))
	}

	fn consume(&mut self, toktype: TokenType) -> Result<Token, Error> {
		if !self.is_at_end() && self.tokens[self.current].toktype == toktype {
			self.current += 1;
			return Ok(self.tokens[self.current - 1].clone());
		}

		Err(Error::fatal(format!("expected {toktype}").as_str(), None))
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
