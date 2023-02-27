use std::{env, error};

mod cmdline;
mod lexer;
mod token;

use lexer::{Lexer, LexerOptions, TokenGenerator};

fn main() -> Result<(), Box<dyn error::Error>> {
    let input = cmdline::parse_args(env::args())?;

    let source = input.chars().collect::<Vec<_>>();
    let lexer = Lexer::new(TokenGenerator::new(&source), LexerOptions::default());

    let tokens = lexer.lex()?;

    println!("tokens:");
    for token in tokens {
        println!("{token:?}");
    }

    Ok(())
}
