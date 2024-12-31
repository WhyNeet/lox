pub mod keywords;

use std::cell::RefCell;

use error::InterpreterError;
use keywords::KEYWORDS;

use crate::{
    error::{ScannerError, ScannerErrorKind, ScannerResult},
    token::{token_literal::TokenLiteral, token_type::TokenType, Token},
};

pub struct Scanner {
    source: String,
    start: RefCell<usize>,
    current: RefCell<usize>,
    line: RefCell<usize>,
    tokens: RefCell<Vec<Token>>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            start: RefCell::new(0),
            current: RefCell::new(0),
            line: RefCell::new(1),
            tokens: RefCell::new(vec![]),
        }
    }

    pub fn scan_tokens(&self) -> ScannerResult<()> {
        while !self.is_at_end() {
            *self.start.borrow_mut() = self.current();
            self.scan_token()?;
        }

        self.tokens
            .borrow_mut()
            .push(Token::new(TokenType::EOF, String::new(), self.line(), None));

        Ok(())
    }

    fn scan_token(&self) -> ScannerResult<()> {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            ':' => self.add_token(TokenType::Colon),
            '*' => self.add_token(TokenType::Star),
            '?' => self.add_token(TokenType::Question),
            '!' => self.add_token(if self.match_char('=') {
                TokenType::BangEqual
            } else {
                TokenType::Bang
            }),
            '=' => self.add_token(if self.match_char('=') {
                TokenType::EqualEqual
            } else {
                TokenType::Equal
            }),
            '>' => self.add_token(if self.match_char('=') {
                TokenType::GreaterEqual
            } else {
                TokenType::Greater
            }),
            '<' => self.add_token(if self.match_char('=') {
                TokenType::LessEqual
            } else {
                TokenType::Less
            }),
            '/' => {
                if self.match_char('/') {
                    // Skip comment
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.match_char('*') {
                    while self.peek() != '*' && self.peek_next() != '/' && !self.is_at_end() {
                        if self.peek() == '\n' {
                            self.advance_lines();
                        }

                        self.advance();
                    }

                    if self.is_at_end() {
                        return Err(
                            self.construct_error(ScannerErrorKind::UnterminatedBlockComment)
                        );
                    } else {
                        // Consume "*/"
                        self.advance_by(2);
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.advance_lines(),
            '"' => self.string()?,
            other => {
                if other.is_ascii_digit() {
                    self.number();
                } else if other.is_ascii_alphabetic() {
                    self.identifier();
                } else {
                    return Err(self.construct_error(ScannerErrorKind::UnexpectedCharacter(other)));
                }
            }
        };

        Ok(())
    }

    fn identifier(&self) {
        while self.peek().is_ascii_alphanumeric() {
            self.advance();
        }

        let value = self.source[self.start()..self.current()].to_string();
        if let Some(token_type) = KEYWORDS.get(&value) {
            self.add_token(*token_type);
        } else {
            self.add_token(TokenType::Identifier);
        }
    }

    fn number(&self) {
        while self.peek().is_ascii_digit() && !self.is_at_end() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            // Consume the "."
            self.advance();

            while self.peek().is_ascii_digit() && !self.is_at_end() {
                self.advance();
            }
        }

        let value = self.source[self.start()..self.current()].parse().unwrap();
        self.add_literal_token(TokenType::Number, Some(TokenLiteral::Number(value)));
    }

    fn string(&self) -> ScannerResult<()> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.advance_lines();
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(self.construct_error(ScannerErrorKind::UnterminatedString));
        }

        // Consume closing '"'
        self.advance();

        let value = self.source[(self.start() + 1)..(self.current() - 1)].to_string();
        self.add_literal_token(TokenType::String, Some(TokenLiteral::String(value)));

        Ok(())
    }

    fn peek_next(&self) -> char {
        if self.current() + 1 >= self.source.len() {
            '\0'
        } else {
            self.source_index(self.current() + 1)
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source_index(self.current())
        }
    }

    fn advance_lines(&self) {
        *self.line.borrow_mut() += 1;
    }

    fn match_char(&self, expected: char) -> bool {
        if self.is_at_end() || self.source_index(self.current()) != expected {
            false
        } else {
            *self.current.borrow_mut() += 1;
            true
        }
    }

    fn advance(&self) -> char {
        let char = self.source.as_bytes()[self.current()] as char;

        self.advance_by(1);

        char
    }

    fn advance_by(&self, advance: usize) {
        *self.current.borrow_mut() += advance;
    }

    fn add_token(&self, token_type: TokenType) {
        self.add_literal_token(token_type, None);
    }

    fn add_literal_token(&self, token_type: TokenType, literal: Option<TokenLiteral>) {
        let lexeme = self.source[self.start()..self.current()].to_string();

        self.tokens
            .borrow_mut()
            .push(Token::new(token_type, lexeme, self.line(), literal))
    }

    fn is_at_end(&self) -> bool {
        self.current() >= self.source.len()
    }

    fn source_index(&self, idx: usize) -> char {
        self.source.as_bytes()[idx] as char
    }

    fn start(&self) -> usize {
        *self.start.borrow()
    }

    fn current(&self) -> usize {
        *self.current.borrow()
    }

    fn line(&self) -> usize {
        *self.line.borrow()
    }
}

impl Scanner {
    pub fn tokens(self) -> Vec<Token> {
        self.tokens.take()
    }

    fn construct_error(&self, kind: ScannerErrorKind) -> InterpreterError<ScannerError> {
        InterpreterError::new(ScannerError::new(kind, self.line()))
    }
}
