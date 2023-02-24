mod lexer;
mod token;

use lexer::{Lexer, LexerOptions, TokenGenerator};
use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    let validity_period = "20Y+3M-1W Week WeekY shkfjshfk  +4D";
    println!("{}", &validity_period);

    let source = validity_period.chars().collect::<Vec<_>>();
    let lexer = Lexer::new(TokenGenerator::new(&source), LexerOptions::default());

    lexer.lex()?;

    Ok(())
}
