
struct Constant {
	v: Literal
}

impl Expr for Constant {}
impl Eval for Constant {
	fn eval(&self) -> Literal {
		self.v.clone()
	}
}

struct Group {
	v: Box<dyn Expr>
}

impl Expr for Group {}
impl Eval for Group {
	fn eval(&self) -> Literal {
		self.v.eval()
	}
}

struct Unary {
	v: Box<dyn Expr>,
	tok: Token,
}

impl Expr for Unary {}
impl Eval for Unary {
	fn eval(&self) -> Literal {
		use TokenType::*;
		
		match self.tok.typing {
			Minus => Literal::sub(Literal::FloatType(0.0), self.v.eval()),
			Bang => todo!(),
			_ => unimplemented!()
		}
	}
}

struct Binary {
	v1: Box<dyn Expr>,
	tok: Token,
	v2: Box<dyn Expr>
}

impl Expr for Binary {}
impl Eval for Binary {
	fn eval(&self) -> Literal {
		use TokenType::*;
		
		match self.tok.typing {
			Plus => Literal::sum(self.v1.eval(), self.v2.eval()),
			Minus => Literal::sub(self.v1.eval(), self.v2.eval()),
			Star => Literal::mul(self.v1.eval(), self.v2.eval()),
			Slash => Literal::div(self.v1.eval(), self.v2.eval()),
			_ => unimplemented!()
		}
	}
}

trait Expr: Eval {}
trait Eval {
	fn eval(&self) -> Literal;
}

struct Scanner<'a> {
	source: String,
	tokens: Vec<Token>,
	error_prone: &'a mut Erroring,
	
	start: usize,
	current: usize,
	line: usize
}

impl<'a> Scanner<'a> {
	fn new(source: String, error_prone: &'a mut Erroring) -> Self {
		Self { source, tokens: Vec::new(), error_prone, start: 0, current: 0, line: 1 }
	}

	fn is_at_end(&self) -> bool { self.current >= self.source.len() }
	
	fn scan_tokens(&mut self) -> Vec<Token> {
		while !self.is_at_end() {
			self.start = self.current;
			self.scan_token();
		}

		self.tokens.push(Token { typing: TokenType::Eof, lexeme: String::from(""), literal: Literal::None, line: self.line });
		self.tokens.clone()
	}

	fn scan_token(&mut self) {
		let c: char = self.advance();
		match c {
			'(' => self.add_token_wrap(TokenType::LeftParen),
			')' => self.add_token_wrap(TokenType::RightParen),
			'{' => self.add_token_wrap(TokenType::LeftBrace),
			'}' => self.add_token_wrap(TokenType::RightBrace),
			',' => self.add_token_wrap(TokenType::Comma),
			'.' => self.add_token_wrap(TokenType::Dot),
			'-' => self.add_token_wrap(TokenType::Minus),
			'+' => self.add_token_wrap(TokenType::Plus),
			';' => self.add_token_wrap(TokenType::Semicolon),
			'*' => self.add_token_wrap(TokenType::Star),
			'!' => {
				let state: TokenType;
				if self.switch('=') {
					state = TokenType::BangEqual;
				} else {
					state = TokenType::Bang;
				}
				self.add_token_wrap(state);
			},
			'=' => {
				let state: TokenType;
				if self.switch('=') {
					state = TokenType::EqualEqual;
				} else {
					state = TokenType::Equal;
				}
				self.add_token_wrap(state);
			},
			'<' => {
				let state: TokenType;
				if self.switch('=') {
					state = TokenType::LessEqual;
				} else {
					state = TokenType::Less;
				}
				self.add_token_wrap(state);
			},
			'>' => {
				let state: TokenType;
				if self.switch('=') {
					state = TokenType::GreaterEqual;
				} else {
					state = TokenType::Greater;
				}
				self.add_token_wrap(state);
			},
			'/' => {
				if self.switch('/') {
					while self.peek() != '\n' && !self.is_at_end() {
						self.advance();
					}
				} else if self.switch('*') {
					let mut nest = 1;
					while nest != 0 && !self.is_at_end() {
						if self.peek() == '/' && self.peek_next() == '*' {
							nest += 1;
							self.advance();
						} else if self.peek() == '*' && self.peek_next() == '/' {
							nest -= 1;
							self.advance();
						}
						
						self.advance();
					}
				} else {
					self.add_token_wrap(TokenType::Slash);
				}
			},
			' ' => (), '\r' => (), '\t' => (), '\n' => { self.line += 1 },
			'"' => self.string(),
			_ => {
				if c.is_digit(10) {
					self.number();
				} else if c.is_alphabetic() {
					self.identifier();
				} else {	
					self.error_prone.error(self.line, "Unexpected character");
				}
			}
		}
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

		return None
	}
	
	fn identifier(&mut self) {
		while self.peek().is_alphanumeric() || self.peek() == '_' { self.advance(); }

		let word = self.get_substring(None, None);
		let Some(kind) = self.keyword_map(&word) else {
			self.add_token_wrap(TokenType::Identifier);
			return;
		};

		self.add_token_wrap(kind);
	}
	
	fn number(&mut self) {
		while self.peek().is_digit(10) { self.advance(); }

		if self.peek() == '.' && self.peek_next().is_digit(10) {
			self.advance();

			while self.peek().is_digit(10) { self.advance(); }
		}

		self.add_token(TokenType::Number, Literal::FloatType(
			self.get_substring(None, None).parse::<f64>().unwrap()
		));
	}
	
	fn string(&mut self) {
		while self.peek() != '"' && !self.is_at_end() {
			if self.peek() == '\n' { self.line += 1; }
			self.advance();
		}

		if self.is_at_end() {
			self.error_prone.error(self.line, "Unterminated string!");
		}

		self.advance();

		let val = self.get_substring(Some(self.start + 1), Some(self.current - 1));
		self.add_token(TokenType::String, Literal::StringType(val));
	}

	fn peek_next(&self) -> char {
		if self.current + 1 >= self.source.len() { return '\0' }
		self.get_char(self.current + 1)
	}
	
	fn peek(&self) -> char {
		if self.is_at_end() { return '\0'; }
		return self.get_char(self.current);
	}

	fn get_char(&self, nth: usize) -> char {
		let c = self.source.chars().nth(nth);
		match c {
			Some(v) => return v,
			None => panic!("Unexpected behaviour")
		}
	}
	
	fn switch(&mut self, expected: char) -> bool {
		if self.is_at_end() { return false; }
		let c = self.get_char(self.current);
		if c != expected { return false; }
		
		self.current += 1;
		true
	}

	fn add_token_wrap(&mut self, typing: TokenType) {
		self.add_token(typing, Literal::None);
	}

	fn add_token(&mut self, typing: TokenType, literal: Literal) {
		let lexeme: String = self.get_substring(None, None);
		self.tokens.push( Token { typing, lexeme, literal, line: self.line } );
	}

	fn get_substring(&self, start: Option<usize>, end: Option<usize>) -> String {
		let (Some(start), Some(end)) = (start, end) else {
			return self.source[self.start..self.current].to_string();
		};

		return self.source[start..end].to_string();
	}

	fn advance(&mut self) -> char {
		let c = self.get_char(self.current);
		self.current += 1;
		c
	}
}

#[derive(Clone, Copy)]
enum TokenType {
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

enum Literal {
	StringType(String),
	FloatType(f64),
	BoolType(bool),
	None
}

impl Literal {
	fn sum(l1: Literal, l2: Literal) -> Literal {
		use Literal::*;
		
		match (l1, l2) {
			(StringType(v1), StringType(v2)) => StringType(format!("{}{}", v1, v2)),
			(FloatType(v1), FloatType(v2)) => FloatType(v1 + v2),
			(StringType(v1), FloatType(v2)) => StringType(v1 + v2.to_string().as_str()),
			(FloatType(v1), StringType(v2)) => StringType(v1.to_string() + v2.as_str()),
			_ => unimplemented!()
		}
	}

	fn sub(l1: Literal, l2: Literal) -> Literal {
		use Literal::*;
		
		match (l1, l2) {
			(StringType(v1), StringType(v2)) => panic!("Cannot subtract two strings!"),
			(FloatType(v1), FloatType(v2)) => FloatType(v1 - v2),
			(StringType(v1), FloatType(v2)) => panic!("Cannot subtract string and float!"),
			(FloatType(v1), StringType(v2)) => panic!("Cannot subtract float and string!"),
			_ => unimplemented!()
		}
	}

	fn mul(l1: Literal, l2: Literal) -> Literal {
		use Literal::*;
		
		match (l1, l2) {
			(StringType(v1), StringType(v2)) => panic!("Cannot multiplicate two strings!"),
			(FloatType(v1), FloatType(v2)) => FloatType(v1 * v2),
			(StringType(v1), FloatType(v2)) => panic!("Cannot multiplicate string and float!"),
			(FloatType(v1), StringType(v2)) => panic!("Cannot multiplicate float and string!"),
			_ => unimplemented!()
		}
	}

	fn div(l1: Literal, l2: Literal) -> Literal {
		use Literal::*;
		
		match (l1, l2) {
			(StringType(v1), StringType(v2)) => panic!("Cannot divide two strings!"),
			(FloatType(v1), FloatType(v2)) => FloatType(v1 / v2),
			(StringType(v1), FloatType(v2)) => panic!("Cannot divide string and float!"),
			(FloatType(v1), StringType(v2)) => panic!("Cannot divide float and string!"),
			_ => unimplemented!()
		}
	}
}

impl Clone for Literal {
	fn clone(&self) -> Self {
		use Literal::*;
		
		match self {
			StringType(v) => StringType(v.clone()),
			FloatType(v) => FloatType(v.clone()),
			BoolType(v) => BoolType(v.clone()),
			None => None
		}
	}
}

struct Token {
	typing: TokenType,
	lexeme: String,
	literal: Literal,
	line: usize
}

impl Token {
	fn new(typing: TokenType, lexeme: String, literal: Literal, line: usize) -> Self {
		Self { typing, lexeme, literal, line }
	}
}

impl ToString for Token {
	fn to_string(&self) -> String {
		let mut temporary = String::new();
		temporary.push_str(&(self.typing as i32).to_string());
		temporary.push_str(" - ");
		temporary.push_str(&self.lexeme);
		temporary.push_str(" - ");
		match &self.literal {
			Literal::StringType(v) => temporary.push_str(&v),
			Literal::FloatType(v) => temporary.push_str(&v.to_string()),
			Literal::BoolType(v) => temporary.push_str(&v.to_string()),
			Literal::None => temporary.push_str(""),
		}

		temporary
	}
}

impl Clone for Token {
	fn clone(&self) -> Self {
		let literal;
		match &self.literal {
			Literal::StringType(v) => literal = Literal::StringType(v.clone()),
			Literal::FloatType(v) => literal = Literal::FloatType(v.clone()),
			Literal::BoolType(v) => literal = Literal::BoolType(v.clone()),
			Literal::None => literal = Literal::None
		}
		
		Token { typing: self.typing.clone(), lexeme: self.lexeme.clone(), literal, line: self.line }
	}
}

struct Erroring {
	had_error: bool,
}

impl Erroring {
	fn check_error(&self) -> bool { self.had_error }
	fn reload(&mut self) { self.had_error = false; }
	
	fn new() -> Self { Self { had_error: false } }
	
	fn error(&mut self, line: usize, msg: &str) {
		self.report(line, "".to_string(), msg);
	}

	fn report(&mut self, line: usize, place: String, msg: &str) {
		println!("[line {line}] Error{place}: {msg}");
		self.had_error = true;
	}
}

struct Lll {
	error_prone: Erroring,
}

impl Lll {

	fn new() -> Self {
		Self { error_prone: Erroring::new() }
	}

	fn run(&mut self, source: &String) {
		let mut scanner = Scanner::new(source.clone(), &mut self.error_prone);
		let tokens = scanner.scan_tokens();

		for i in tokens {
			println!("{:?}", i.to_string());
		}
	}

	fn run_file(&mut self, filename: &String) -> std::io::Result<()> {
		let Ok(text) = std::fs::read_to_string(filename) else {
			panic!("FATAL: не нашёл на воровской дороге файл");
		};

		self.run(&text);

		if self.error_prone.check_error() { std::process::exit(65); }
		
		Ok(())
	}

	fn run_promt(&mut self) -> std::io::Result<()> {
		let mut buf = String::new();
		let stdin = std::io::stdin();

		loop {
			let Ok(res) = stdin.read_line(&mut buf) else {
				panic!("FATAL: not valid utf-8")
			};

			if res == 0 {
				return Ok(());
			}
			
			print!("> ");
			
			self.run(&buf);
			self.error_prone.reload();
		}
	}
	
}



fn main() -> std::io::Result<()> {
	
    // let args: Vec<_> = std::env::args().collect();

	// let mut l = Lll::new();
	
	// if args.len() > 2 {
	// 	println!("USAGE: lll [script]");
	// 	std::process::exit(64);
	// } else if args.len() == 2 {
	// 	l.run_file(&args[1]).expect("FATAL: something happen out there! he wrote some fucking bad code look at him");
	// } else {
	// 	l.run_promt().unwrap();
	// }

	Ok(())
}

