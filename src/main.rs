
struct Printer;
impl Visitor<String> for Printer {
	
	fn visit_expr(&mut self, expr: &Expr) -> String {
		match *expr {
			Expr::Literal(ref v) => {
				match v {
					Literal::StringType(s) => s.clone(),
					Literal::FloatType(f) => f.to_string(),
					Literal::Identifier => todo!(),
					Literal::None => "nil".to_string(),
				}
			},
			Expr::Group(ref v) => format!("(grp {})", self.visit_expr(v)),
			Expr::Unary(ref tok, ref v) => {
				match tok {
					TokenType::Minus => format!("(- {})", self.visit_expr(v)),
					_ => unimplemented!()
				}
			},
			Expr::Add(ref v1, ref v2) => format!("(+ {} {})", self.visit_expr(v1), self.visit_expr(v2)),
			Expr::Sub(ref v1, ref v2) => format!("(- {} {})", self.visit_expr(v1), self.visit_expr(v2)),
			Expr::Mul(ref v1, ref v2) => format!("(* {} {})", self.visit_expr(v1), self.visit_expr(v2)),
			Expr::Div(ref v1, ref v2) => format!("(/ {} {})", self.visit_expr(v1), self.visit_expr(v2)),
		}
	}
}

trait Visitor<T> {
	fn visit_expr(&mut self, expr: &Expr) -> T;
}

enum Expr {
	Literal(Literal),
	Group(Box<Expr>),
	Unary(TokenType, Box<Expr>),
	Add(Box<Expr>, Box<Expr>),
	Sub(Box<Expr>, Box<Expr>),
	Mul(Box<Expr>, Box<Expr>),
	Div(Box<Expr>, Box<Expr>),
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
	Identifier,
	None
}

impl Clone for Literal {
	fn clone(&self) -> Self {
		use Literal::*;
		
		match self {
			StringType(v) => StringType(v.clone()),
			FloatType(v) => FloatType(v.clone()),
			Identifier => Identifier,
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
			Literal::Identifier => todo!(),
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
			Literal::Identifier => literal = Literal::Identifier,
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

	let mut debug: Printer = Printer {};
	println!("{}", debug.visit_expr(&Expr::Add(
		Box::new(Expr::Unary(TokenType::Minus,
							 Box::new(Expr::Literal(Literal::FloatType(30.0))))),
		Box::new(Expr::Div(
			Box::new(Expr::Group(
				Box::new(Expr::Mul(
					Box::new(Expr::Literal(Literal::FloatType(53.2))),
					Box::new(Expr::Literal(Literal::FloatType(2321.2)))
				))
			)),
			Box::new(Expr::Add(
				Box::new(Expr::Literal(Literal::FloatType(232.22))),
				Box::new(Expr::Literal(Literal::FloatType(555.2222)))
			))
		))
	)));
	
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

