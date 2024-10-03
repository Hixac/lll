
use super::token::Token;

#[derive(Debug)]
pub struct Error {
	pub token: Option<Token>,
	pub msg: String
}

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match &self.token {
			Some(token) => write!(f, "FATAL: {}, line {}, token {}", self.msg, token.line, token.toktype),
			None => write!(f, "FATAL: {}", self.msg)
		}
	}
}

impl Error {
	pub fn new(msg: &str, token: Option<&Token>) -> Self {
		match token {
			Some(v) => Self { token: Some(v.clone()), msg: msg.to_string() },
			None => Self { token: None, msg: msg.to_string() }
		}
		
	}
}
