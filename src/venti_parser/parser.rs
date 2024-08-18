use crate::errors::VentiError;
use crate::venti_lexer::token::Token;
use crate::venti_parser::ast::{BinOp, Expr, Statement};
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
            Some(Token::Venti) => self.function_or_variable(),
            Some(Token::Print) => self.print_statement(),
            _ => Err(VentiError::SyntaxError(format!(
                "Unexpected token: {:?}",
                self.current_token()
            ))),
        }
    }

    fn variable_declaration(&mut self) -> Result<Statement, VentiError> {
        self.advance(); // Consume 'venti'

        // Match the identifier
        if let Some(Token::Identifier) = self.current_token() {
            self.advance(); // Consume the identifier
            let identifier = "some_identifier".to_string(); // Placeholder for actual identifier name

            // Match the equals sign
            if let Some(Token::Equals) = self.current_token() {
                self.advance(); // Consume '='

                // Parse the expression assigned to the variable
                let value = self.expression()?;

                // Match the semicolon
                if let Some(Token::Semicolon) = self.current_token() {
                    self.advance(); // Consume ';'
                                    // Return the variable declaration statement
                    return Ok(Statement::VariableDeclaration { identifier, value });
                } else {
                    return Err(VentiError::SyntaxError(
                        "Expected ';' at the end of variable declaration.".to_string(),
                    ));
                }
            } else {
                return Err(VentiError::SyntaxError(
                    "Expected '=' in variable declaration.".to_string(),
                ));
            }
        } else {
            return Err(VentiError::SyntaxError(
                "Expected identifier in variable declaration.".to_string(),
            ));
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
                self.advance(); // Consume the number literal token
                                // Here you need to assign or convert the actual value from the token
                let number = "0"
                    .parse::<i64>()
                    .map_err(|_| VentiError::SyntaxError("Invalid number".to_string()))?;
                Ok(Expr::Number(number))
            }
            Some(Token::StringLiteral) => {
                self.advance(); // Consume the string literal token
                let value = "some_string_value".to_string(); // Placeholder
                Ok(Expr::String(value))
            }
            Some(Token::Identifier) => {
                self.advance(); // Consume the identifier token
                let name = "some_identifier".to_string(); // Placeholder
                Ok(Expr::Identifier(name))
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

    fn advance_and_get(&mut self) -> Option<Token> {
        self.advance();
        self.current_token().cloned()
    }

    fn function_or_variable(&mut self) -> Result<Statement, VentiError> {
        if let Some(Token::Identifier) = self.current_token() {
            let identifier = "some_identifier".to_string(); // Placeholder for actual identifier

            self.advance();
            if let Some(Token::LParen) = self.current_token() {
                self.advance(); // Consume '('
                                // Parse function call arguments gere
                let args = Vec::new(); // Placeholder for actual args parsing
                if let Some(Token::RParen) = self.current_token() {
                    self.advance();
                    return Ok(Statement::FunctionCall { identifier, args });
                } else {
                }
            }
            Err(VentiError::SyntaxError(
                "Invalid function or variable statement".to_string(),
            ))
        }
    }
}
