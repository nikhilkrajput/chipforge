//! Comprehensive Verilog lexer with IEEE 1364-2005 support

use chipforge_common::{Error, Result};
use chipforge_common::location::Location;

/// Verilog token types
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Module,
    EndModule,
    Input,
    Output,
    Inout,
    Wire,
    Reg,
    Integer,
    Real,
    Time,
    Parameter,
    Localparam,
    Assign,
    Always,
    Initial,
    Begin,
    End,
    If,
    Else,
    Case,
    Casex,
    Casez,
    Default,
    EndCase,
    For,
    While,
    Repeat,
    Forever,
    Function,
    EndFunction,
    Task,
    EndTask,
    Posedge,
    Negedge,
    Or,
    Generate,
    EndGenerate,
    Genvar,
    Signed,
    Unsigned,

    // SystemVerilog keywords
    Logic,
    Bit,
    Byte,
    Shortint,
    Int,
    Longint,
    Typedef,
    Enum,
    Struct,
    Union,
    Package,
    EndPackage,
    Interface,
    EndInterface,
    Modport,

    // Operators
    Plus,           // +
    Minus,          // -
    Star,           // *
    Slash,          // /
    Percent,        // %
    Equal,          // =
    EqualEqual,     // ==
    NotEqual,       // !=
    EqualEqualEqual, // ===
    NotEqualEqual,  // !==
    Less,           // <
    LessEqual,      // <=
    Greater,        // >
    GreaterEqual,   // >=
    LogicalAnd,     // &&
    LogicalOr,      // ||
    LogicalNot,     // !
    BitwiseAnd,     // &
    BitwiseOr,      // |
    BitwiseXor,     // ^
    BitwiseNot,     // ~
    BitwiseNand,    // ~&
    BitwiseNor,     // ~|
    BitwiseXnor,    // ~^, ^~
    ShiftLeft,      // <<
    ShiftRight,     // >>
    ArithShiftLeft, // <<<
    ArithShiftRight, // >>>
    Question,       // ?
    Colon,          // :

    // Delimiters
    LParen,         // (
    RParen,         // )
    LBracket,       // [
    RBracket,       // ]
    LBrace,         // {
    RBrace,         // }
    Semicolon,      // ;
    Comma,          // ,
    Dot,            // .
    Hash,           // #
    At,             // @

    // Literals
    Identifier(String),
    Number(NumberLiteral),
    StringLiteral(String),

    // Special
    Eof,
}

/// Number literal representation
#[derive(Debug, Clone, PartialEq)]
pub struct NumberLiteral {
    pub size: Option<u32>,
    pub base: NumberBase,
    pub value: String,
    pub is_signed: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NumberBase {
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
}

/// Verilog lexer with position tracking
pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    line: usize,
    column: usize,
    file_id: usize,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer
    pub fn new(input: &'a str, _filename: &str) -> Self {
        Lexer {
            input,
            position: 0,
            line: 1,
            column: 1,
            file_id: 0, // TODO: use file table
        }
    }

    /// Get current location
    pub fn location(&self) -> Location {
        Location::new(self.file_id, self.line, self.column)
    }

    /// Get the next token
    pub fn next_token(&mut self) -> Result<Token> {
        self.skip_whitespace_and_comments()?;

        if self.is_eof() {
            return Ok(Token::Eof);
        }

        let ch = self.current_char()?;

        match ch {
            // Single character tokens
            '(' => {
                self.advance()?;
                Ok(Token::LParen)
            }
            ')' => {
                self.advance()?;
                Ok(Token::RParen)
            }
            '[' => {
                self.advance()?;
                Ok(Token::LBracket)
            }
            ']' => {
                self.advance()?;
                Ok(Token::RBracket)
            }
            '{' => {
                self.advance()?;
                Ok(Token::LBrace)
            }
            '}' => {
                self.advance()?;
                Ok(Token::RBrace)
            }
            ';' => {
                self.advance()?;
                Ok(Token::Semicolon)
            }
            ',' => {
                self.advance()?;
                Ok(Token::Comma)
            }
            '.' => {
                self.advance()?;
                Ok(Token::Dot)
            }
            '#' => {
                self.advance()?;
                Ok(Token::Hash)
            }
            '@' => {
                self.advance()?;
                Ok(Token::At)
            }
            '?' => {
                self.advance()?;
                Ok(Token::Question)
            }
            ':' => {
                self.advance()?;
                Ok(Token::Colon)
            }
            '%' => {
                self.advance()?;
                Ok(Token::Percent)
            }

            // Multi-character operators
            '+' => {
                self.advance()?;
                Ok(Token::Plus)
            }
            '-' => {
                self.advance()?;
                Ok(Token::Minus)
            }
            '*' => {
                self.advance()?;
                Ok(Token::Star)
            }
            '/' => {
                self.advance()?;
                Ok(Token::Slash)
            }
            '=' => {
                self.advance()?;
                if self.peek_char()? == Some('=') {
                    self.advance()?;
                    if self.peek_char()? == Some('=') {
                        self.advance()?;
                        Ok(Token::EqualEqualEqual)
                    } else {
                        Ok(Token::EqualEqual)
                    }
                } else {
                    Ok(Token::Equal)
                }
            }
            '!' => {
                self.advance()?;
                if self.peek_char()? == Some('=') {
                    self.advance()?;
                    if self.peek_char()? == Some('=') {
                        self.advance()?;
                        Ok(Token::NotEqualEqual)
                    } else {
                        Ok(Token::NotEqual)
                    }
                } else {
                    Ok(Token::LogicalNot)
                }
            }
            '<' => {
                self.advance()?;
                if self.peek_char()? == Some('=') {
                    self.advance()?;
                    Ok(Token::LessEqual)
                } else if self.peek_char()? == Some('<') {
                    self.advance()?;
                    if self.peek_char()? == Some('<') {
                        self.advance()?;
                        Ok(Token::ArithShiftLeft)
                    } else {
                        Ok(Token::ShiftLeft)
                    }
                } else {
                    Ok(Token::Less)
                }
            }
            '>' => {
                self.advance()?;
                if self.peek_char()? == Some('=') {
                    self.advance()?;
                    Ok(Token::GreaterEqual)
                } else if self.peek_char()? == Some('>') {
                    self.advance()?;
                    if self.peek_char()? == Some('>') {
                        self.advance()?;
                        Ok(Token::ArithShiftRight)
                    } else {
                        Ok(Token::ShiftRight)
                    }
                } else {
                    Ok(Token::Greater)
                }
            }
            '&' => {
                self.advance()?;
                if self.peek_char()? == Some('&') {
                    self.advance()?;
                    Ok(Token::LogicalAnd)
                } else {
                    Ok(Token::BitwiseAnd)
                }
            }
            '|' => {
                self.advance()?;
                if self.peek_char()? == Some('|') {
                    self.advance()?;
                    Ok(Token::LogicalOr)
                } else {
                    Ok(Token::BitwiseOr)
                }
            }
            '^' => {
                self.advance()?;
                if self.peek_char()? == Some('~') {
                    self.advance()?;
                    Ok(Token::BitwiseXnor)
                } else {
                    Ok(Token::BitwiseXor)
                }
            }
            '~' => {
                self.advance()?;
                match self.peek_char()? {
                    Some('&') => {
                        self.advance()?;
                        Ok(Token::BitwiseNand)
                    }
                    Some('|') => {
                        self.advance()?;
                        Ok(Token::BitwiseNor)
                    }
                    Some('^') => {
                        self.advance()?;
                        Ok(Token::BitwiseXnor)
                    }
                    _ => Ok(Token::BitwiseNot)
                }
            }

            // String literals
            '"' => self.read_string_literal(),

            // Numbers
            '0'..='9' => self.read_number(),
            '\'' => self.read_based_number(),

            // Identifiers and keywords
            'a'..='z' | 'A'..='Z' | '_' => self.read_identifier(),

            // System tasks and compiler directives
            '$' => self.read_system_task(),
            '`' => self.read_compiler_directive(),

            _ => Err(Error::parse(
                format!("Unexpected character: '{}'", ch),
                self.location(),
            )),
        }
    }

    fn is_eof(&self) -> bool {
        self.position >= self.input.len()
    }

    fn current_char(&self) -> Result<char> {
        self.input[self.position..]
            .chars()
            .next()
            .ok_or_else(|| Error::parse("Unexpected end of file".to_string(), self.location()))
    }

    fn peek_char(&self) -> Result<Option<char>> {
        Ok(self.input[self.position..].chars().nth(1))
    }

    fn advance(&mut self) -> Result<()> {
        if let Some(ch) = self.input[self.position..].chars().next() {
            self.position += ch.len_utf8();
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
        Ok(())
    }

    fn skip_whitespace_and_comments(&mut self) -> Result<()> {
        loop {
            if self.is_eof() {
                break;
            }

            let ch = self.current_char()?;

            if ch.is_whitespace() {
                self.advance()?;
            } else if ch == '/' {
                // Check for comments
                if let Some(next) = self.peek_char()? {
                    if next == '/' {
                        // Single-line comment
                        self.skip_single_line_comment()?;
                    } else if next == '*' {
                        // Multi-line comment
                        self.skip_multi_line_comment()?;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        Ok(())
    }

    fn skip_single_line_comment(&mut self) -> Result<()> {
        // Skip //
        self.advance()?;
        self.advance()?;

        // Skip until newline or EOF
        while !self.is_eof() {
            let ch = self.current_char()?;
            self.advance()?;
            if ch == '\n' {
                break;
            }
        }
        Ok(())
    }

    fn skip_multi_line_comment(&mut self) -> Result<()> {
        // Skip /*
        self.advance()?;
        self.advance()?;

        // Skip until */
        while !self.is_eof() {
            let ch = self.current_char()?;
            if ch == '*' {
                self.advance()?;
                if !self.is_eof() && self.current_char()? == '/' {
                    self.advance()?;
                    break;
                }
            } else {
                self.advance()?;
            }
        }
        Ok(())
    }

    fn read_identifier(&mut self) -> Result<Token> {
        let start = self.position;

        // First character already validated
        self.advance()?;

        // Continue while alphanumeric, underscore, or dollar sign
        while !self.is_eof() {
            let ch = self.current_char()?;
            if ch.is_alphanumeric() || ch == '_' || ch == '$' {
                self.advance()?;
            } else {
                break;
            }
        }

        let text = &self.input[start..self.position];

        // Check if it's a keyword
        let token = match text {
            "module" => Token::Module,
            "endmodule" => Token::EndModule,
            "input" => Token::Input,
            "output" => Token::Output,
            "inout" => Token::Inout,
            "wire" => Token::Wire,
            "reg" => Token::Reg,
            "integer" => Token::Integer,
            "real" => Token::Real,
            "time" => Token::Time,
            "parameter" => Token::Parameter,
            "localparam" => Token::Localparam,
            "assign" => Token::Assign,
            "always" => Token::Always,
            "initial" => Token::Initial,
            "begin" => Token::Begin,
            "end" => Token::End,
            "if" => Token::If,
            "else" => Token::Else,
            "case" => Token::Case,
            "casex" => Token::Casex,
            "casez" => Token::Casez,
            "default" => Token::Default,
            "endcase" => Token::EndCase,
            "for" => Token::For,
            "while" => Token::While,
            "repeat" => Token::Repeat,
            "forever" => Token::Forever,
            "function" => Token::Function,
            "endfunction" => Token::EndFunction,
            "task" => Token::Task,
            "endtask" => Token::EndTask,
            "posedge" => Token::Posedge,
            "negedge" => Token::Negedge,
            "or" => Token::Or,
            "generate" => Token::Generate,
            "endgenerate" => Token::EndGenerate,
            "genvar" => Token::Genvar,
            "signed" => Token::Signed,
            "unsigned" => Token::Unsigned,

            // SystemVerilog
            "logic" => Token::Logic,
            "bit" => Token::Bit,
            "byte" => Token::Byte,
            "shortint" => Token::Shortint,
            "int" => Token::Int,
            "longint" => Token::Longint,
            "typedef" => Token::Typedef,
            "enum" => Token::Enum,
            "struct" => Token::Struct,
            "union" => Token::Union,
            "package" => Token::Package,
            "endpackage" => Token::EndPackage,
            "interface" => Token::Interface,
            "endinterface" => Token::EndInterface,
            "modport" => Token::Modport,

            _ => Token::Identifier(text.to_string()),
        };

        Ok(token)
    }

    fn read_number(&mut self) -> Result<Token> {
        let start = self.position;

        // Read decimal number or sized number
        while !self.is_eof() {
            let ch = self.current_char()?;
            if ch.is_numeric() {
                self.advance()?;
            } else {
                break;
            }
        }

        // Check if this is a sized number (e.g., 8'hFF)
        if !self.is_eof() && self.current_char()? == '\'' {
            let size_str = &self.input[start..self.position];
            let size = size_str.parse::<u32>().map_err(|_| {
                Error::parse(
                    format!("Invalid number size: {}", size_str),
                    self.location(),
                )
            })?;

            self.advance()?; // Skip '

            return self.read_based_number_value(Some(size));
        }

        // Just a decimal number
        let text = &self.input[start..self.position];
        Ok(Token::Number(NumberLiteral {
            size: None,
            base: NumberBase::Decimal,
            value: text.to_string(),
            is_signed: false,
        }))
    }

    fn read_based_number(&mut self) -> Result<Token> {
        self.advance()?; // Skip initial '
        self.read_based_number_value(None)
    }

    fn read_based_number_value(&mut self, size: Option<u32>) -> Result<Token> {
        let is_signed = if !self.is_eof() && self.current_char()? == 's' {
            self.advance()?;
            true
        } else {
            false
        };

        let base = if self.is_eof() {
            return Err(Error::parse(
                "Expected base specifier after '".to_string(),
                self.location(),
            ));
        } else {
            let ch = self.current_char()?;
            self.advance()?;
            match ch {
                'b' | 'B' => NumberBase::Binary,
                'o' | 'O' => NumberBase::Octal,
                'd' | 'D' => NumberBase::Decimal,
                'h' | 'H' => NumberBase::Hexadecimal,
                _ => {
                    return Err(Error::parse(
                        format!("Invalid base specifier: '{}'", ch),
                        self.location(),
                    ));
                }
            }
        };

        let start = self.position;
        while !self.is_eof() {
            let ch = self.current_char()?;
            let valid = match base {
                NumberBase::Binary => ch == '0' || ch == '1' || ch == 'x' || ch == 'X' || ch == 'z' || ch == 'Z' || ch == '_',
                NumberBase::Octal => ch.is_digit(8) || ch == 'x' || ch == 'X' || ch == 'z' || ch == 'Z' || ch == '_',
                NumberBase::Decimal => ch.is_numeric() || ch == '_',
                NumberBase::Hexadecimal => ch.is_ascii_hexdigit() || ch == 'x' || ch == 'X' || ch == 'z' || ch == 'Z' || ch == '_',
            };

            if valid {
                self.advance()?;
            } else {
                break;
            }
        }

        let value = self.input[start..self.position].to_string();

        Ok(Token::Number(NumberLiteral {
            size,
            base,
            value,
            is_signed,
        }))
    }

    fn read_string_literal(&mut self) -> Result<Token> {
        self.advance()?; // Skip opening "

        let mut string = String::new();

        while !self.is_eof() {
            let ch = self.current_char()?;

            if ch == '"' {
                self.advance()?;
                return Ok(Token::StringLiteral(string));
            } else if ch == '\\' {
                self.advance()?;
                if !self.is_eof() {
                    let escaped = self.current_char()?;
                    self.advance()?;
                    match escaped {
                        'n' => string.push('\n'),
                        't' => string.push('\t'),
                        'r' => string.push('\r'),
                        '\\' => string.push('\\'),
                        '"' => string.push('"'),
                        _ => {
                            string.push('\\');
                            string.push(escaped);
                        }
                    }
                }
            } else {
                string.push(ch);
                self.advance()?;
            }
        }

        Err(Error::parse(
            "Unterminated string literal".to_string(),
            self.location(),
        ))
    }

    fn read_system_task(&mut self) -> Result<Token> {
        let start = self.position;
        self.advance()?; // Skip $

        while !self.is_eof() {
            let ch = self.current_char()?;
            if ch.is_alphanumeric() || ch == '_' {
                self.advance()?;
            } else {
                break;
            }
        }

        let text = &self.input[start..self.position];
        Ok(Token::Identifier(text.to_string()))
    }

    fn read_compiler_directive(&mut self) -> Result<Token> {
        let start = self.position;
        self.advance()?; // Skip `

        while !self.is_eof() {
            let ch = self.current_char()?;
            if ch.is_alphanumeric() || ch == '_' {
                self.advance()?;
            } else {
                break;
            }
        }

        let text = &self.input[start..self.position];
        Ok(Token::Identifier(text.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keywords() {
        let input = "module endmodule input output wire reg";
        let mut lexer = Lexer::new(input, "test.v");

        assert_eq!(lexer.next_token().unwrap(), Token::Module);
        assert_eq!(lexer.next_token().unwrap(), Token::EndModule);
        assert_eq!(lexer.next_token().unwrap(), Token::Input);
        assert_eq!(lexer.next_token().unwrap(), Token::Output);
        assert_eq!(lexer.next_token().unwrap(), Token::Wire);
        assert_eq!(lexer.next_token().unwrap(), Token::Reg);
    }

    #[test]
    fn test_operators() {
        let input = "+ - * / == != === !== && || ! & | ^ ~";
        let mut lexer = Lexer::new(input, "test.v");

        assert_eq!(lexer.next_token().unwrap(), Token::Plus);
        assert_eq!(lexer.next_token().unwrap(), Token::Minus);
        assert_eq!(lexer.next_token().unwrap(), Token::Star);
        assert_eq!(lexer.next_token().unwrap(), Token::Slash);
        assert_eq!(lexer.next_token().unwrap(), Token::EqualEqual);
        assert_eq!(lexer.next_token().unwrap(), Token::NotEqual);
        assert_eq!(lexer.next_token().unwrap(), Token::EqualEqualEqual);
        assert_eq!(lexer.next_token().unwrap(), Token::NotEqualEqual);
        assert_eq!(lexer.next_token().unwrap(), Token::LogicalAnd);
        assert_eq!(lexer.next_token().unwrap(), Token::LogicalOr);
        assert_eq!(lexer.next_token().unwrap(), Token::LogicalNot);
        assert_eq!(lexer.next_token().unwrap(), Token::BitwiseAnd);
        assert_eq!(lexer.next_token().unwrap(), Token::BitwiseOr);
        assert_eq!(lexer.next_token().unwrap(), Token::BitwiseXor);
        assert_eq!(lexer.next_token().unwrap(), Token::BitwiseNot);
    }

    #[test]
    fn test_numbers() {
        let input = "42 8'hFF 4'b1010 16'd255";
        let mut lexer = Lexer::new(input, "test.v");

        let tok1 = lexer.next_token().unwrap();
        assert!(matches!(tok1, Token::Number(_)));

        let tok2 = lexer.next_token().unwrap();
        if let Token::Number(num) = tok2 {
            assert_eq!(num.size, Some(8));
            assert_eq!(num.base, NumberBase::Hexadecimal);
            assert_eq!(num.value, "FF");
        } else {
            panic!("Expected number token");
        }
    }

    #[test]
    fn test_comments() {
        let input = "module // comment\n/* block\ncomment */ endmodule";
        let mut lexer = Lexer::new(input, "test.v");

        assert_eq!(lexer.next_token().unwrap(), Token::Module);
        assert_eq!(lexer.next_token().unwrap(), Token::EndModule);
    }

    #[test]
    fn test_string_literal() {
        let input = r#""hello world""#;
        let mut lexer = Lexer::new(input, "test.v");

        assert_eq!(
            lexer.next_token().unwrap(),
            Token::StringLiteral("hello world".to_string())
        );
    }
}
