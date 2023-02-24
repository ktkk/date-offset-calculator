use super::Offset;
use crate::token::{TimeUnit, Token};

pub struct TokenGenerator<'source> {
    content: &'source [char],
    offset: usize,
}

impl<'source> TokenGenerator<'source> {
    pub fn new(content: &'source [char]) -> Self {
        Self { content, offset: 0 }
    }

    pub fn trim_left(&mut self) {
        while !self.content.is_empty() && self.content[0].is_whitespace() {
            self.content = &self.content[1..];
            self.offset += 1;
        }
    }

    pub fn chop(&mut self, amount: usize) -> &'source [char] {
        let token = &self.content[0..amount];
        self.content = &self.content[amount..];

        self.offset += amount;

        token
    }

    pub fn chop_while<P>(&mut self, mut predicate: P) -> &'source [char]
    where
        P: FnMut(&char) -> bool,
    {
        let mut amount = 0;
        while amount < self.content.len() && predicate(&self.content[amount]) {
            amount += 1;
        }

        self.chop(amount)
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.trim_left();

        if self.content.is_empty() {
            return None;
        }

        match self.content[0] {
            c if c.is_numeric() => {
                let token: String = self.chop_while(|x| x.is_numeric()).iter().collect();

                let token = if let Ok(duration) = token.parse::<u32>() {
                    Token::Duration(duration)
                } else {
                    Token::Unknown(token)
                };

                Some(token)
            }
            c if c.is_alphabetic() => {
                let token: String = self.chop_while(|x| x.is_alphabetic()).iter().collect();

                let token = match token.as_str() {
                    "Y" | "Year" => Token::Unit(TimeUnit::Year),
                    "M" | "Month" => Token::Unit(TimeUnit::Month),
                    "W" | "Week" => Token::Unit(TimeUnit::Week),
                    "D" | "Day" => Token::Unit(TimeUnit::Day),
                    _ => Token::Unknown(token),
                };

                Some(token)
            }
            '+' => {
                let _ = self.chop(1);

                Some(Token::Plus)
            }
            '-' => {
                let _ = self.chop(1);

                Some(Token::Minus)
            }
            _ => Some(Token::Unknown(self.chop(1).iter().collect())),
        }
    }
}

impl<'source> Iterator for TokenGenerator<'source> {
    type Item = (Token, Offset);

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Some(token) => Some((token, self.offset)),
            None => None,
        }
    }
}
