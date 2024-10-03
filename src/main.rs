
mod lll;
mod lang;
use lang::*;

// Incremental error system, where I just add a &str every error and continue parse
fn main() -> std::process::ExitCode {
	let args: Vec<_> = std::env::args().collect();

	if args.len() > 2 {
		eprintln!("USE: ./lll [source file].");
		eprintln!("INFO: provided args {args:?}");
		return std::process::ExitCode::FAILURE;
	} else if args.len() == 2 {
		let path = std::path::PathBuf::from(&args[1]);
		run_file(&path);
		return std::process::ExitCode::SUCCESS;
	} else {
		run_interactive();
		return std::process::ExitCode::SUCCESS;
	}
}
