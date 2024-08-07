use crate::venti_parser::ast::{BinOp, Expr, Statement};
use crate::venti_lexer::token::Token;
use crate::errors::VentiError;
use std::iter::Peekable;
use std::vec::IntoIter;

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens.into_iter().peekable(),
        }
    }

    fn advance(&mut self) {
        self.tokens.next();
    }

    fn current_token(&mut self) -> Option<&Token> {
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
            _ => Err(VentiError::SyntaxError(format!("Unexpected token: {:?}", self.current_token()))),
        }
    }

    fn variable_declaration(&mut self) -> Result<Statement, VentiError> {
        self.advance(); // consume 'venti'
        let identifier = if let Some(Token::Identifier) = self.current_token() {
            self.advance(); // consume identifier
            if let Some(Token::Equals) = self.current_token() {
                self.advance(); // consume '='
                let value = self.expression()?;
                self.advance(); // consume ';'
                Ok(Statement::VariableDeclaration { identifier: "identifier".to_string(), value })
            } else {
                Err(VentiError::SyntaxError("Expected '='".to_string()))
            }
        } else {
            Err(VentiError::SyntaxError("Expected identifier".to_string()))
        }
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
            Some(Token::NumberLiteral) => {
                if let Some(Token::NumberLiteral(value)) = self.advance_and_get() {
                    let number = value.parse().map_err(|_| VentiError::SyntaxError("Invalid number".to_string()))?;
                    Ok(Expr::Number(number))
                } else {
                    Err(VentiError::SyntaxError("Expected number".to_string()))
                }
            }
            Some(Token::StringLiteral) => {
                if let Some(Token::StringLiteral(value)) = self.advance_and_get() {
                    Ok(Expr::String(value))
                } else {
                    Err(VentiError::SyntaxError("Expected string".to_string()))
                }
            }
            Some(Token::Identifier) => {
                if let Some(Token::Identifier(name)) = self.advance_and_get() {
                    Ok(Expr::Identifier(name))
                } else {
                    Err(VentiError::SyntaxError("Expected identifier".to_string()))
                }
            }
            Some(Token::LParen) => {
                self.advance(); // consume '('
                let expr = self.expression()?;
                if self.current_token() == Some(&Token::RParen) {
                    self.advance(); // consume ')'
                    Ok(expr)
                } else {
                    Err(VentiError::SyntaxError("Expected ')'".to_string()))
                }
            }
            Some(Token::LBracket) => self.parse_array(),
            _ => Err(VentiError::SyntaxError(format!("Unexpected token: {:?}", self.current_token()))),
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

    fn advance_and_get(&mut self) -> Option<Token> {
        self.advance();
        self.current_token().cloned()
    }
}
