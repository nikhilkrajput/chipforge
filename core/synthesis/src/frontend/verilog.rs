//! Verilog parser

use chipforge_common::{Error, Result};
use chipforge_common::location::Location;

/// Verilog lexer token
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// Module keyword
    Module,
    /// Endmodule keyword
    EndModule,
    /// Identifier
    Identifier(String),
    /// Number literal
    Number(i64),
    /// Left parenthesis
    LParen,
    /// Right parenthesis
    RParen,
    /// Semicolon
    Semicolon,
    /// End of file
    Eof,
}

/// Simple Verilog lexer
pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer
    pub fn new(input: &'a str) -> Self {
        Lexer { input, position: 0 }
    }

    /// Get the next token
    pub fn next_token(&mut self) -> Result<Token> {
        self.skip_whitespace();

        if self.position >= self.input.len() {
            return Ok(Token::Eof);
        }

        let ch = self.current_char();

        match ch {
            '(' => {
                self.advance();
                Ok(Token::LParen)
            }
            ')' => {
                self.advance();
                Ok(Token::RParen)
            }
            ';' => {
                self.advance();
                Ok(Token::Semicolon)
            }
            'a'..='z' | 'A'..='Z' | '_' => self.read_identifier(),
            '0'..='9' => self.read_number(),
            _ => Err(Error::parse(
                format!("Unexpected character: {}", ch),
                Location::dummy(),
            )),
        }
    }

    fn current_char(&self) -> char {
        self.input[self.position..].chars().next().unwrap()
    }

    fn advance(&mut self) {
        if let Some(ch) = self.input[self.position..].chars().next() {
            self.position += ch.len_utf8();
        }
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() {
            let ch = self.current_char();
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_identifier(&mut self) -> Result<Token> {
        let start = self.position;
        while self.position < self.input.len() {
            let ch = self.current_char();
            if ch.is_alphanumeric() || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let text = &self.input[start..self.position];
        let token = match text {
            "module" => Token::Module,
            "endmodule" => Token::EndModule,
            _ => Token::Identifier(text.to_string()),
        };

        Ok(token)
    }

    fn read_number(&mut self) -> Result<Token> {
        let start = self.position;
        while self.position < self.input.len() {
            let ch = self.current_char();
            if ch.is_numeric() {
                self.advance();
            } else {
                break;
            }
        }

        let text = &self.input[start..self.position];
        let number = text.parse::<i64>().map_err(|_| {
            Error::parse(
                format!("Invalid number: {}", text),
                Location::dummy(),
            )
        })?;

        Ok(Token::Number(number))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let input = "module test();";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token().unwrap(), Token::Module);
        assert_eq!(lexer.next_token().unwrap(), Token::Identifier("test".to_string()));
        assert_eq!(lexer.next_token().unwrap(), Token::LParen);
        assert_eq!(lexer.next_token().unwrap(), Token::RParen);
        assert_eq!(lexer.next_token().unwrap(), Token::Semicolon);
        assert_eq!(lexer.next_token().unwrap(), Token::Eof);
    }
}
