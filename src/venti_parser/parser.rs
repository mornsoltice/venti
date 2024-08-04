use crate::ast::{BinOp, Expr, Statement};
use crate::lexer::Token;
use std::iter::Peekable;
use std::vec::IntoIter;

use super::ast::Expr;

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

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();
        while self.current_token().is_some() {
            statements.push(self.statement());
        }
        statements
    }

    fn statement(&mut self) -> Statement {
        match self.current_token() {
            Some(Token::Venti) => self.variable_declaration(),
            Some(Token::Print) => self.print_statement(),
            _ => panic!("Unexpected token: {:?}", self.current_token()),
        }
    }

    fn variable_declaration(&mut self) -> Statement {
        self.advance(); // consume 'venti'
        let identifier = if let Some(Token::Identifier(name)) = self.current_token() {
            name.clone()
        } else {
            panic!("Expected identifier");
        };
        self.advance(); // consume identifier
        self.advance(); // consume '='
        let value = self.expression();
        self.advance(); // consume ';'
        Statement::VariableDeclaration { identifier, value }
    }

    fn print_statement(&mut self) -> Statement {
        self.advance(); // consume 'print'
        let value = self.expression();
        self.advance(); // consume ';'
        Statement::Print(value)
    }

    fn expression(&mut self) -> Expr {
        self.term()
    }

    fn term(&mut self) -> Expr {
        let mut left = self.factor();
        while let Some(token) = self.current_token() {
            match token {
                Token::Plus => {
                    self.advance();
                    let right = self.factor();
                    left = Expr::BinaryOp(Box::new(left), BinOp::Add, Box::new(right));
                }
                Token::Minus => {
                    self.advance();
                    let right = self.factor();
                    left = Expr::BinaryOp(Box::new(left), BinOp::Subtract, Box::new(right));
                }
                _ => break,
            }
        }
        left
    }

    fn factor(&mut self) -> Expr {
        let mut left = self.primary();
        while let Some(token) = self.current_token() {
            match token {
                Token::Star => {
                    self.advance();
                    let right = self.primary();
                    left = Expr::BinaryOp(Box::new(left), BinOp::Multiply, Box::new(right));
                }
                Token::Slash => {
                    self.advance();
                    let right = self.primary();
                    left = Expr::BinaryOp(Box::new(left), BinOp::Divide, Box::new(right));
                }
                _ => break,
            }
        }
        left
    }

    fn primary(&mut self) -> Expr {
        match self.current_token() {
            Some(Token::NumberLiteral(value)) => {
                let number = value.parse().unwrap();
                self.advance();
                Expr::Number(number)
            }
            Some(Token::StringLiteral(value)) => {
                let string = value.clone();
                self.advance();
                Expr::String(string)
            }
            Some(Token::Identifier(name)) => {
                let identifier = name.clone();
                self.advance();
                Expr::Identifier(identifier)
            }
            Some(Token::LParen) => {
                self.advance();
                let expr = self.expression();
                self.advance(); // consume ')'
                expr
            }
            _ => panic!("Unexpected token: {:?}", self.current_token()),
        }
    }

    fn parse_array(&mut self) -> Expr {
        self.advance(); // Consume '['
        let mut elements = Vec::new();
        while self.current_token() != Some(&Token::RBracket) {
            element.push(self.expression());
            if self.current_token() == Sme(&Token::Comma) {
                self.advance(); // consume ','
            }
        }
        self.advance(); // consume ']'
        Expr::Array(elements)
    }
}
