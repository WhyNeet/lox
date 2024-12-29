pub mod error;

use std::cell::RefCell;

use ::error::InterpreterError;
use ast::{expression::Expression, literal::Literal};
use error::{ParserError, ParserErrorKind, ParserResult};
use lexer::token::{token_type::TokenType, Token};

#[derive(Debug, Default)]
pub struct Parser {
    tokens: Vec<Token>,
    current: RefCell<usize>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            ..Self::default()
        }
    }

    pub fn run(&self) -> ParserResult<Expression> {
        self.expression()
    }

    fn expression(&self) -> ParserResult<Expression> {
        self.equality()
    }

    fn equality(&self) -> ParserResult<Expression> {
        let mut expr = self.comparison()?;

        while self.match_token(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().unwrap().try_into().unwrap();
            let right = self.comparison()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn comparison(&self) -> ParserResult<Expression> {
        let mut expr = self.term()?;

        while self.match_token(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().unwrap().try_into().unwrap();
            let right = self.term()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term(&self) -> ParserResult<Expression> {
        let mut expr = self.factor()?;

        while self.match_token(&[TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous().unwrap().try_into().unwrap();
            let right = self.factor()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor(&self) -> ParserResult<Expression> {
        let mut expr = self.unary()?;

        while self.match_token(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().unwrap().try_into().unwrap();
            let right = self.unary()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary(&self) -> ParserResult<Expression> {
        if self.match_token(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().unwrap().try_into().unwrap();
            let right = self.unary()?;
            Ok(Expression::Unary {
                operator,
                right: Box::new(right),
            })
        } else {
            self.primary()
        }
    }

    fn primary(&self) -> ParserResult<Expression> {
        if self.match_token(&[TokenType::False]) {
            return Ok(Expression::Literal(Literal::Boolean(false)));
        }
        if self.match_token(&[TokenType::True]) {
            return Ok(Expression::Literal(Literal::Boolean(true)));
        }
        if self.match_token(&[TokenType::Nil]) {
            return Ok(Expression::Literal(Literal::Nil));
        }
        if self.match_token(&[TokenType::Number]) {
            return Ok(Expression::Literal(Literal::Number(
                self.previous()
                    .unwrap()
                    .literal()
                    .unwrap()
                    .as_number()
                    .unwrap(),
            )));
        }
        if self.match_token(&[TokenType::String]) {
            return Ok(Expression::Literal(Literal::String(
                self.previous()
                    .unwrap()
                    .literal()
                    .unwrap()
                    .as_string()
                    .map(|s| s.to_string())
                    .unwrap(),
            )));
        };

        if self.match_token(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            if !self.match_token(&[TokenType::RightParen]) {
                return Err(self.construct_error(ParserErrorKind::TokenExpected(')')));
            }

            return Ok(Expression::Grouping(Box::new(expr)));
        }

        panic!()
    }

    fn match_token(&self, tokens: &[TokenType]) -> bool {
        for token_type in tokens {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        self.peek()
            .map(|token| token.token_type() == token_type)
            .unwrap_or(false)
    }

    fn is_at_end(&self) -> bool {
        self.peek()
            .map(|token| *token.token_type() == TokenType::EOF)
            .unwrap_or(false)
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current())
    }

    fn advance(&self) -> &Token {
        self.advance_by(1);
        self.previous().unwrap()
    }

    fn previous(&self) -> Option<&Token> {
        self.tokens.get(self.current() - 1)
    }

    fn advance_by(&self, advance: usize) {
        *self.current.borrow_mut() += advance;
    }

    fn current(&self) -> usize {
        *self.current.borrow()
    }
}

impl Parser {
    fn construct_error(&self, kind: ParserErrorKind) -> InterpreterError<ParserError> {
        InterpreterError::new(ParserError::new(kind))
    }
}
