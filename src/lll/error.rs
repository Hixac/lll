
use super::token::Token;
	
#[derive(Debug)]
enum ErrorType {
	Warn,
	Fatal
}

#[derive(Debug)]
pub struct Error {
	token: Option<Token>,
	msg: String,
	typing: ErrorType
}

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let buf;
		match &self.token {
			Some(token) => buf = format!("{}, line {}, token {}", self.msg, token.line + 1, token.toktype),
			None => buf = format!("{}", self.msg)
		};
		match &self.typing {
			ErrorType::Warn => write!(f, "WARN: {buf}"),
			ErrorType::Fatal => {
				write!(f, "FATAL: {buf}")
			},
		}
	}
}

impl Error {
	pub fn fatal(msg: &str, token: Option<&Token>) -> Self {
		match token {
			Some(v) => Self { token: Some(v.clone()), msg: msg.to_string(), typing: ErrorType::Fatal },
			None => Self { token: None, msg: msg.to_string(), typing: ErrorType::Fatal }
		}
	}

	pub fn warn(msg: &str, token: Option<&Token>) -> Self {
		match token {
			Some(v) => Self { token: Some(v.clone()), msg: msg.to_string(), typing: ErrorType::Warn },
			None => Self { token: None, msg: msg.to_string(), typing: ErrorType::Warn }
		}
	}
}
