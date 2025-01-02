pub mod error;

use std::{cell::RefCell, rc::Rc};

use ::error::InterpreterError;
use ast::{expression::Expression, literal::Literal, statement::Statement};
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

    pub fn run(&self) -> ParserResult<Vec<Rc<Statement>>> {
        self.program()
    }

    fn program(&self) -> ParserResult<Vec<Rc<Statement>>> {
        let mut statements = vec![];

        while !self.is_at_end() {
            statements.push(Rc::new(self.declaration()?));
        }

        Ok(statements)
    }

    fn declaration(&self) -> ParserResult<Statement> {
        if self.match_token(&[TokenType::Var]) {
            self.var_decl()
        } else if self.match_token(&[TokenType::Fun]) {
            self.fun_decl()
        } else {
            self.statement()
        }
    }

    fn fun_decl(&self) -> ParserResult<Statement> {
        if !self.match_token(&[TokenType::Identifier]) {
            return Err(self.construct_error(ParserErrorKind::IdentifierExpected));
        }

        let identifier = self.previous().unwrap().lexeme().to_string();

        if !self.match_token(&[TokenType::LeftParen]) {
            return Err(self.construct_error(ParserErrorKind::TokenExpected('(')));
        }

        let parameters = self.parameters()?;

        if !self.match_token(&[TokenType::LeftBrace]) {
            return Err(self.construct_error(ParserErrorKind::TokenExpected('{')));
        }

        let execute = self.block()?;

        Ok(Statement::FunctionDeclaration {
            identifier,
            parameters,
            execute: Box::new(execute),
        })
    }

    fn parameters(&self) -> ParserResult<Vec<String>> {
        let mut parameters = vec![];

        while !self.is_at_end() && !self.check(&TokenType::RightParen) {
            if !self.match_token(&[TokenType::Identifier]) {
                return Err(self.construct_error(ParserErrorKind::IdentifierExpected));
            }

            let identifier = self.previous().unwrap().lexeme().to_string();

            parameters.push(identifier);
        }

        if !self.match_token(&[TokenType::RightParen]) {
            return Err(self.construct_error(ParserErrorKind::TokenExpected(')')));
        }

        Ok(parameters)
    }

    fn var_decl(&self) -> ParserResult<Statement> {
        if !self.match_token(&[TokenType::Identifier]) {
            return Err(self.construct_error(ParserErrorKind::IdentifierExpected));
        }

        let identifier = self.previous().unwrap().lexeme().to_string();

        if !self.match_token(&[TokenType::Equal]) {
            return Err(self.construct_error(ParserErrorKind::TokenExpected('=')));
        }

        let expression = self.expression()?;

        if !self.match_token(&[TokenType::Semicolon]) {
            return Err(self.construct_error(ParserErrorKind::TokenExpected(';')));
        }

        Ok(Statement::VariableDeclaration {
            identifier,
            expression,
        })
    }

    fn statement(&self) -> ParserResult<Statement> {
        if self.match_token(&[TokenType::Print]) {
            self.print_stmt()
        } else if self.match_token(&[TokenType::LeftBrace]) {
            self.block()
        } else if self.match_token(&[TokenType::If]) {
            self.if_stmt()
        } else if self.match_token(&[TokenType::While]) {
            self.while_stmt()
        } else if self.match_token(&[TokenType::Break]) {
            self.break_stmt()
        } else if self.match_token(&[TokenType::Continue]) {
            self.continue_stmt()
        } else {
            self.expr_stmt()
        }
    }

    fn break_stmt(&self) -> ParserResult<Statement> {
        if !self.match_token(&[TokenType::Semicolon]) {
            Err(self.construct_error(ParserErrorKind::TokenExpected(';')))
        } else {
            Ok(Statement::Break)
        }
    }

    fn continue_stmt(&self) -> ParserResult<Statement> {
        if !self.match_token(&[TokenType::Semicolon]) {
            Err(self.construct_error(ParserErrorKind::TokenExpected(';')))
        } else {
            Ok(Statement::Continue)
        }
    }

    fn while_stmt(&self) -> ParserResult<Statement> {
        let condition = self.expression()?;

        if !self.match_token(&[TokenType::LeftBrace]) {
            return Err(self.construct_error(ParserErrorKind::TokenExpected('{')));
        }

        let block = self.block()?;

        Ok(Statement::While {
            condition,
            block: Box::new(block),
        })
    }

    fn if_stmt(&self) -> ParserResult<Statement> {
        let condition = self.expression()?;
        if !self.match_token(&[TokenType::LeftBrace]) {
            return Err(self.construct_error(ParserErrorKind::TokenExpected('{')));
        }
        let then = self.block()?;

        let alternative = if self.match_token(&[TokenType::Else]) {
            if !self.check(&TokenType::If) {
                if !self.match_token(&[TokenType::LeftBrace]) {
                    return Err(self.construct_error(ParserErrorKind::TokenExpected('{')));
                }

                Some(self.block()?)
            } else {
                Some(self.statement()?)
            }
        } else {
            None
        };

        Ok(Statement::Conditional {
            condition,
            then: Box::new(then),
            alternative: alternative.map(Box::new),
        })
    }

    fn block(&self) -> ParserResult<Statement> {
        let mut statements = vec![];

        while !self.is_at_end() && !self.check(&TokenType::RightBrace) {
            statements.push(Rc::new(self.declaration()?));
        }

        if !self.match_token(&[TokenType::RightBrace]) {
            Err(self.construct_error(ParserErrorKind::TokenExpected('}')))
        } else {
            Ok(Statement::Block(statements))
        }
    }

    fn expr_stmt(&self) -> ParserResult<Statement> {
        let expression = self.expression()?;

        if self.match_token(&[TokenType::Semicolon]) {
            Ok(Statement::Expression(expression))
        } else {
            Err(self.construct_error(ParserErrorKind::TokenExpected(';')))
        }
    }

    fn print_stmt(&self) -> ParserResult<Statement> {
        let expression = self.expression()?;

        if self.match_token(&[TokenType::Semicolon]) {
            Ok(Statement::Print(expression))
        } else {
            Err(self.construct_error(ParserErrorKind::TokenExpected(';')))
        }
    }

    fn expression(&self) -> ParserResult<Expression> {
        self.assignment()
    }

    fn assignment(&self) -> ParserResult<Expression> {
        let expr = self.ternary()?;

        if !self.match_token(&[TokenType::Equal]) {
            return Ok(expr);
        }

        let identifier = match expr {
            Expression::Identifier(identifier) => identifier,
            _ => return Err(self.construct_error(ParserErrorKind::IdentifierExpected)),
        };

        let expression = self.expression()?;

        Ok(Expression::Assignment {
            identifier,
            expression: Box::new(expression),
        })
    }

    fn ternary(&self) -> ParserResult<Expression> {
        let mut expr = self.logic_or()?;

        if self.match_token(&[TokenType::Question]) {
            let then = self.logic_or()?;
            if !self.match_token(&[TokenType::Colon]) {
                return Err(self.construct_error(ParserErrorKind::TokenExpected(':')));
            }

            let alternative = self.logic_or()?;

            expr = Expression::Conditional {
                condition: Box::new(expr),
                then: Box::new(then),
                alternative: Box::new(alternative),
            };
        }

        Ok(expr)
    }

    fn logic_or(&self) -> ParserResult<Expression> {
        let mut expr = self.logic_and()?;

        while self.match_token(&[TokenType::Or]) {
            let operator = self.previous().unwrap().try_into().unwrap();
            let right = self.logic_and()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn logic_and(&self) -> ParserResult<Expression> {
        let mut expr = self.equality()?;

        while self.match_token(&[TokenType::And]) {
            let operator = self.previous().unwrap().try_into().unwrap();
            let right = self.equality()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn equality(&self) -> ParserResult<Expression> {
        if self.check_many(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            return Err(self.construct_error(ParserErrorKind::MissingLeftHandOperand));
        }

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
        if self.check_many(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            return Err(self.construct_error(ParserErrorKind::MissingLeftHandOperand));
        }

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
        if self.check_many(&[TokenType::Plus, TokenType::Minus]) {
            return Err(self.construct_error(ParserErrorKind::MissingLeftHandOperand));
        }

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
        if self.check_many(&[TokenType::Slash, TokenType::Star]) {
            return Err(self.construct_error(ParserErrorKind::MissingLeftHandOperand));
        }

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
            self.call()
        }
    }

    fn call(&self) -> ParserResult<Expression> {
        let mut expr = self.primary()?;

        while self.match_token(&[TokenType::LeftParen]) {
            let arguments = self.arguments()?;
            expr = Expression::FunctionInvokation {
                callee: Box::new(expr),
                arguments,
            };

            if !self.match_token(&[TokenType::RightParen]) {
                return Err(self.construct_error(ParserErrorKind::TokenExpected(')')));
            }
        }

        Ok(expr)
    }

    fn arguments(&self) -> ParserResult<Vec<Expression>> {
        let mut arguments = vec![];

        while !self.is_at_end() && !self.check(&TokenType::RightParen) {
            let expr = self.expression()?;
            arguments.push(expr);
        }

        Ok(arguments)
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
        if self.match_token(&[TokenType::Identifier]) {
            return Ok(Expression::Identifier(
                self.previous().unwrap().lexeme().to_string(),
            ));
        }

        if self.match_token(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            if !self.match_token(&[TokenType::RightParen]) {
                return Err(self.construct_error(ParserErrorKind::TokenExpected(')')));
            }

            return Ok(Expression::Grouping(Box::new(expr)));
        }

        Err(self.construct_error(ParserErrorKind::ExpressionExprected))
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

    fn synchronize(&self) {
        self.advance();

        while !self.is_at_end() {
            if *self.previous().unwrap().token_type() == TokenType::Semicolon {
                break;
            }

            match self.advance().token_type() {
                TokenType::Class
                | TokenType::If
                | TokenType::Var
                | TokenType::While
                | TokenType::Print
                | TokenType::For
                | TokenType::Return
                | TokenType::Fun => break,
                _ => (),
            }
        }
    }

    fn check(&self, token_type: &TokenType) -> bool {
        self.peek()
            .map(|token| token.token_type() == token_type)
            .unwrap_or(false)
    }

    fn check_many(&self, token_type: &[TokenType]) -> bool {
        self.peek()
            .map(|token| {
                token_type
                    .iter()
                    .find(|&token_type| token.token_type() == token_type)
                    .is_some()
            })
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
