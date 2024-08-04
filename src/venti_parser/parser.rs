use crate::ast::{BinOp, Expr, Statement};
use crate::errors::VentiError;
use crate::lexer::Token;
use std::iter::Peekable;

use std::vec::IntoIter;

pub struct Parser<'a> {
    tokens: Peekable<IntoIter<Token>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens.into_iter().peekable(),
        }
    }

    fn advance(&mut self) {
        self.tokens.next();
    }

    fn current_token(&self) -> Option<&Token> {
        self.tokens.peek()
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, VentiError> {
        let mut statements = Vec::new();
        while self.current_token().is_some() {
            statements.push(self.statement()?);
        }
        Ok(statements)
    }

    fn statement(&mut self) -> Result<Statement, VentiError> {
        match self.current_token() {
            Some(Token::Venti) => self.variable_declaration(),
            Some(Token::Print) => self.print_statement(),
            _ => Err(VentiError::SyntaxError(format!(
                "Unexpected token: {:?}",
                self.current_token()
            ))),
        }
    }

    fn variable_declaration(&mut self) -> Result<Statement, VentiError> {
        self.advance(); // consume 'venti'
        let identifier = if let Some(Token::Identifier(name)) = self.current_token() {
            name.clone()
        } else {
            return Err(VentiError::SyntaxError("Expected identifier".to_string()));
        };
        self.advance(); // consume identifier
        self.advance(); // consume '='
        let value = self.expression()?;
        self.advance(); // consume ';'
        Ok(Statement::VariableDeclaration { identifier, value })
    }

    fn print_statement(&mut self) -> Result<Statement, VentiError> {
        self.advance(); // consume 'printventi'
        let value = self.expression()?;
        self.advance(); // consume ';'
        Ok(Statement::Print(value))
    }

    fn expression(&mut self) -> Result<Expr, VentiError> {
        self.term()
    }

    fn term(&mut self) -> Result<Expr, VentiError> {
        let mut left = self.factor()?;
        while let Some(token) = self.current_token() {
            match token {
                Token::Plus => {
                    self.advance();
                    let right = self.factor()?;
                    left = Expr::BinaryOp(Box::new(left), BinOp::Add, Box::new(right));
                }
                Token::Minus => {
                    self.advance();
                    let right = self.factor()?;
                    left = Expr::BinaryOp(Box::new(left), BinOp::Subtract, Box::new(right));
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn factor(&mut self) -> Result<Expr, VentiError> {
        let mut left = self.primary()?;
        while let Some(token) = self.current_token() {
            match token {
                Token::Star => {
                    self.advance();
                    let right = self.primary()?;
                    left = Expr::BinaryOp(Box::new(left), BinOp::Multiply, Box::new(right));
                }
                Token::Slash => {
                    self.advance();
                    let right = self.primary()?;
                    left = Expr::BinaryOp(Box::new(left), BinOp::Divide, Box::new(right));
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn primary(&mut self) -> Result<Expr, VentiError> {
        match self.current_token() {
            Some(Token::NumberLiteral(value)) => {
                let number = value
                    .parse()
                    .map_err(|_| VentiError::SyntaxError("Invalid number".to_string()))?;
                self.advance();
                Ok(Expr::Number(number))
            }
            Some(Token::StringLiteral(value)) => {
                let string = value.clone();
                self.advance();
                Ok(Expr::String(string))
            }
            Some(Token::Identifier(name)) => {
                let identifier = name.clone();
                self.advance();
                Ok(Expr::Identifier(identifier))
            }
            Some(Token::LParen) => {
                self.advance();
                let expr = self.expression()?;
                if self.current_token() == Some(&Token::RParen) {
                    self.advance(); // consume ')'
                    Ok(expr)
                } else {
                    Err(VentiError::SyntaxError("Expected ')'".to_string()))
                }
            }
            Some(Token::LBracket) => self.parse_array(),
            _ => Err(VentiError::SyntaxError(format!(
                "Unexpected token: {:?}",
                self.current_token()
            ))),
        }
    }

    fn parse_array(&mut self) -> Result<Expr, VentiError> {
        self.advance(); // consume '['
        let mut elements = Vec::new();
        while self.current_token() != Some(&Token::RBracket) {
            elements.push(self.expression()?);
            if self.current_token() == Some(&Token::Comma) {
                self.advance(); // consume ','
            }
        }
        self.advance(); // consume ']'
        Ok(Expr::Array(elements))
    }
}

