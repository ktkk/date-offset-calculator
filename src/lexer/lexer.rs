use std::{error, fmt};

use crate::token::Token;

pub type Offset = usize;

pub struct Lexer<T>
where
    T: Iterator<Item = (Token, Offset)>,
{
    token_generator: T,
    options: LexerOptions,
}

impl<T> Lexer<T>
where
    T: Iterator<Item = (Token, Offset)>,
{
    pub fn new(token_generator: T, options: LexerOptions) -> Self {
        Self {
            token_generator,
            options,
        }
    }

    pub fn lex(self) -> Result<(), LexingError> {
        for (token, offset) in self.token_generator {
            if self.options.fail_on_unknown {
                if let Token::Unknown(token) = token {
                    let offset = offset - token.len();

                    return Err(LexingError::UnknownToken { token, offset });
                }
            }

            println!("{token:?}");
        }

        Ok(())
    }
}

pub struct LexerOptions {
    pub fail_on_unknown: bool,
}

impl Default for LexerOptions {
    fn default() -> Self {
        Self {
            fail_on_unknown: false,
        }
    }
}

pub enum LexingError {
    UnknownToken { token: String, offset: Offset },
}

impl fmt::Debug for LexingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UnknownToken { token, offset } => {
                write!(f, "Unknown token \"{token}\" at offset {offset}",)
            }
        }
    }
}

impl fmt::Display for LexingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for LexingError {}
