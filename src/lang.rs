
use crate::lll::lexer::Lexer;
use crate::lll::token::Token;
use crate::lll::token::TokenType;
use crate::lll::token::Literal;
use crate::lll::parse::Parser;
use crate::lll::interpreter::interpret;

// maybe need to wrap around or not
fn run(source: String) {
	let mut lexer = Lexer::new(source);

	let mut tokens = lexer.scan_tokens();
	tokens.push(Token { literal: Literal::Nil, toktype: TokenType::Eof, place: 0, line: 0 } );

	for i in &tokens {
		println!("{}", i.to_string());
	}

	let mut parser = Parser::new(tokens);
	let mut stmts = parser.parse();

	match &mut stmts {
		Some(v) => interpret(v),
		None => {
			eprintln!("FATAL: parser caused error")
		}
	}
}

pub fn run_file(path: &std::path::PathBuf) {
	let Ok(text) = std::fs::read_to_string(path) else {
		panic!("FATAL: не нашёл на воровской дороге файл");
	};

	run(text);
}

// This thing cannot work here smh, but other places would
pub fn run_interactive() {
	let stdin = std::io::stdin();

	loop {
		let mut buf = String::new();
		let Ok(res) = stdin.read_line(&mut buf) else {
			panic!("FATAL: not valid utf-8")
		};

		if res == 0 {
			return;
		}
		
		print!("> ");
		
		run(buf);
	}
}
