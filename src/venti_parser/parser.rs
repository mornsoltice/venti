use crate::errors::VentiError;
use crate::venti_lexer::token::Token;
use crate::venti_parser::ast::{BinOp, Expr, Statement};
use std::iter::Peekable;
use std::vec::IntoIter;

/// A Parser For the Venti Programming Lang
///
/// The `parser` is responsible for converting a sequence of tokens into an AST
pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
}

impl Parser {
    /// Crate a new `Parser` with the given tokens
    ///
    /// # Arguments
    ///
    /// * `tokens` - A vector of `Token` objects representing the token to parse
    ///
    /// # Returns
    ///
    /// A new instance of `Parser`.
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens.into_iter().peekable(),
        }
    }

    /// Advance the current token without advancing the iterator
    fn advance(&mut self) {
        self.tokens.next();
    }

    /// Returns the current token without advancing the iterator
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the current `Token`, or `None` if there are no more tokens
    fn current_token(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

    /// Parses the entire input and produces a vector of a statements
    ///
    /// # Returns
    ///
    /// A `Result` containing either a vector of `Statement` objects or a `VentiError` if parsing fails.
    pub fn parse(&mut self) -> Result<Vec<Statement>, VentiError> {
        let mut statements = Vec::new();
        while self.current_token().is_some() {
            statements.push(self.statement()?);
        }
        Ok(statements)
    }

    /// Parses a single statement
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Statement` or a `VentiError` if the statement is invalid.
    fn statement(&mut self) -> Result<Statement, VentiError> {
        match self.current_token() {
            Some(Token::Venti) => {
                self.advance(); // Consume 'venti'
                self.variable_declaration()
            }
            Some(Token::Print) => {
                self.advance(); // Consume 'printventi'
                self.print_statement()
            }
            Some(Token::Identifier(_)) => self.function_or_variable(),
            _ => Err(VentiError::SyntaxError(format!(
                "Unexpected token: {:?}",
                self.current_token()
            ))),
        }
    }

    /// Parses a variable declaration statement.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Statement::VariableDeclaration` or a `VentiError` if the declaration is invalid.
    fn variable_declaration(&mut self) -> Result<Statement, VentiError> {
        // Match the identifier
        let identifier = if let Some(Token::Identifier(id)) = self.current_token() {
            id.clone()
        } else {
            return Err(VentiError::SyntaxError(
                "Expected identifier in variable declaration.".to_string(),
            ));
        };

        self.advance(); // Consume identifier

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
    }

    /// Parses a print statement.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Statement::Print` or a `VentiError` if the print statement is invalid.
    fn print_statement(&mut self) -> Result<Statement, VentiError> {
        // Parse the expression to be printed
        let value = self.expression()?;

        // Consume the semicolon
        if let Some(Token::Semicolon) = self.current_token() {
            self.advance(); // Consume ';'
            return Ok(Statement::Print(value));
        } else {
            return Err(VentiError::SyntaxError(
                "Expected ';' at the end of print statement.".to_string(),
            ));
        }
    }

    /// Parses an expression, starting with the term.
    ///
    /// # Returns
    ///
    /// A `Result` containing an `Expr` or a `VentiError` if the expression is invalid.
    fn expression(&mut self) -> Result<Expr, VentiError> {
        self.term()
    }

    /// Parses a term, which may include addition and subtraction.
    ///
    /// # Returns
    ///
    /// A `Result` containing an `Expr` or a `VentiError` if the term is invalid.
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

    /// Parses a factor, which may include multiplication and division.
    ///
    /// # Returns
    ///
    /// A `Result` containing an `Expr` or a `VentiError` if the factor is invalid.
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

    /// Parses a primary expression, which can be a number, string, identifier, or parenthesized expression.
    ///
    /// # Returns
    ///
    /// A `Result` containing an `Expr` or a `VentiError` if the primary expression is invalid.
    fn primary(&mut self) -> Result<Expr, VentiError> {
        match self.current_token() {
            Some(Token::NumberLiteral(n)) => {
                self.advance(); // Consume the number literal token
                Ok(Expr::Number(n.clone()))
            }
            Some(Token::StringLiteral(s)) => {
                self.advance(); // Consume the string literal token
                Ok(Expr::String(s.clone()))
            }
            Some(Token::Identifier(id)) => {
                self.advance(); // Consume the identifier token
                Ok(Expr::Identifier(id.clone()))
            }
            Some(Token::LParen) => {
                self.advance(); // consume '('
                let expr = self.expression()?;
                if let Some(Token::RParen) = self.current_token() {
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

    /// Parses an array literal.
    ///
    /// # Returns
    ///
    /// A `Result` containing an `Expr::Array` or a `VentiError` if the array is invalid.
    fn parse_array(&mut self) -> Result<Expr, VentiError> {
        self.advance(); // consume '['
        let mut elements = Vec::new();
        while self.current_token() != Some(&Token::RBracket) {
            elements.push(self.expression()?);
            if let Some(Token::Comma) = self.current_token() {
                self.advance(); // consume ','
            } else if let Some(Token::RBracket) = self.current_token() {
                break;
            }
        }
        self.advance(); // consume ']'
        Ok(Expr::Array(elements))
    }

    /// Parses either a function call or a variable assignment.
    ///
    /// # Returns
    ///
    /// A `Result` containing either a `Statement::FunctionCall` or `Statement::VariableAssignment`, or a `VentiError` if invalid.
    fn function_or_variable(&mut self) -> Result<Statement, VentiError> {
        let identifier = if let Some(Token::Identifier(id)) = self.current_token() {
            id.clone()
        } else {
            return Err(VentiError::SyntaxError("Expected identifier".to_string()));
        };

        self.advance(); // Consume identifier

        if let Some(Token::LParen) = self.current_token() {
            self.advance(); // Consume '('
            let mut args = Vec::new();
            while self.current_token() != Some(&Token::RParen) {
                args.push(self.expression()?);
                if let Some(Token::Comma) = self.current_token() {
                    self.advance(); // Consume ','
                } else if let Some(Token::RParen) = self.current_token() {
                    break;
                }
            }
            self.advance(); // Consume ')'
            return Ok(Statement::FunctionCall { identifier, args });
        }

        // Handle variable assignment if no '(' is found
        let value = self.expression()?;
        if let Some(Token::Semicolon) = self.current_token() {
            self.advance(); // Consume ';'
            return Ok(Statement::VariableAssignment { identifier, value });
        }

        Err(VentiError::SyntaxError(
            "Expected ';' after variable assignment.".to_string(),
        ))
    }
}

