
use super::parse::Stmt;

pub fn interpret(stmts: &mut Vec<Box<dyn Stmt>>) {
	for i in stmts {
		execute(i);
	}
}

fn execute(stmt: &mut Box<dyn Stmt>) -> Result<(), &'static str> {
	stmt.emit();

	Ok(())
}
