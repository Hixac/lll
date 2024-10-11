use std::collections::HashMap;

use super::error::Error;
use super::parse::Stmt;
use super::parse::Expr;
use super::token::Token;
use super::token::Literal;
use super::token::TokenType;

struct Environment {
	vals: HashMap<String, Literal>
}

impl Environment {
	pub fn new() -> Self {
		Self { vals: HashMap::new() }
	}
	
	pub fn get(&self, name: &Token) -> Result<Literal, Error> {
		match &name.literal {
			Literal::Identifier(v) => {
				match self.vals.get(v) {
					Some(v) => Ok(v.clone()),
					None => Err(Error::new("variable identifier not found", Some(&name)))
				}
			},
			_ => Err(Error::new("variable identifier was literal", Some(&name)))
		}
	}
	
	pub fn define(&mut self, name: String, val: Literal) {
		self.vals.insert(name, val);
	}

	pub fn assign(&mut self, name: &Token, val: &Literal) -> Result<(), Error> {
		match &name.literal {
			Literal::Identifier(v) => {
				let Some(key_val) = self.vals.get_mut(v) else {
					return Err(Error::new("trying to change non-existing variable", Some(&name)))
				};
				*key_val = val.clone();
				Ok(())
			}
			_ => Err(Error::new("trying to change non-existing variable", Some(&name)))
		}
	}
}

pub struct Interpreter {
	env: Environment
}

impl Interpreter {
	pub fn new() -> Self {
		Self { env: Environment::new() }
	}
	
	pub fn interpret(&mut self, stmts: &mut Vec<Stmt>) -> Result<(), Error> {	
		for i in stmts {
			match self.execute_stmt(i) {
				Ok(()) => (),
				Err(e) => println!("{e}")
			}
		}

		Ok(())
	}

	fn execute_stmt(&mut self, stmt: &mut Stmt) -> Result<(), Error> {
		match stmt {
			Stmt::Variable(t, v) => self.var(&t, &v),
			Stmt::Print(v) => Ok(self.print(&v)?),
			Stmt::Expression(v) => {
				self.execute_expr(v);
				Ok(())
			}
		}
	}

	fn var(&mut self, t: &Token, v: &Expr) -> Result<(), Error> {
		match &t.literal {
			Literal::Identifier(name) => {
				let expr = self.execute_expr(&v)?;
				self.env.define(name.clone(), expr);
				return Ok(())
			},
			_ => return Err(Error {token: Some(t.clone()), msg: "wrong the hell literal".to_string()}),
		}
	}
	
	fn print(&mut self, v: &Expr) -> Result<(), Error> {
		use Literal::*;
		match self.execute_expr(&v)? {
			String(v) => {
				print!("{}", v.replace("\\n", "\n"));
			},
			Float(v) => print!("{v}"),
			Bool(v) => print!("{v}"),
			Nil => print!("nil"),
			Identifier(_) => print!("identifier"),
		}

		Ok(())
	}

	fn execute_expr(&mut self, expr: &Expr) -> Result<Literal, Error> {
		use Expr::*;
		match expr {
			Binary(v1, t, v2) => self.binary(v1, t, v2),
			Unary(t, v) => self.unary(t, v),
			Group(v) => self.execute_expr(&v),
			Variable(t) => self.env.get(t),
			Assign(t, v) => self.assign(&t, &v),
			Constant(v) => Ok(v.clone())
		}
	}

	fn assign(&mut self, t: &Token, expr: &Expr) -> Result<Literal, Error> {
		let val = self.execute_expr(expr)?;
		self.env.assign(&t, &val)?;
		Ok(val)
	}

	fn binary(&mut self, v1: &Box<Expr>, t: &Token, v2: &Box<Expr>) -> Result<Literal, Error> {
		use TokenType::*;
		match &t.toktype {
			Plus => Literal::sum(self.execute_expr(v1)?, self.execute_expr(v2)?),
			Minus => Literal::sub(self.execute_expr(v1)?, self.execute_expr(v2)?),
			Star => Literal::mul(self.execute_expr(v1)?, self.execute_expr(v2)?),
			Slash => Literal::div(self.execute_expr(v1)?, self.execute_expr(v2)?),
			EqualEqual => Literal::eq(self.execute_expr(v1)?, self.execute_expr(v2)?),
			Greater => Literal::gt(self.execute_expr(v1)?, self.execute_expr(v2)?),
			GreaterEqual => Literal::egt(self.execute_expr(v1)?, self.execute_expr(v2)?),
			Less => Literal::lt(self.execute_expr(v1)?, self.execute_expr(v2)?),
			LessEqual => Literal::elt(self.execute_expr(v1)?, self.execute_expr(v2)?),
			_ => Err(Error::new("FATAL: unexpected operator in binary!", Some(&t)))
		}
	}

	fn unary(&mut self, t: &Token, v: &Box<Expr>) -> Result<Literal, Error> {
		use TokenType::*;
		match &t.toktype {
			Minus => Literal::sub(Literal::Float(0.0), self.execute_expr(&v)?),
			Bang => {
				match self.execute_expr(v)? {
					Literal::Bool(b) => Ok(Literal::Bool(!b)),
					_ => Err(Error::new("FATAL: unexpected operator in unary!", Some(&t)))
				}
			},
			_ => Err(Error::new("FATAL: unexpected operator in unary!", Some(&t)))
		}
	}

}
