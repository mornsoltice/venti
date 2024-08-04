use crate::errors::VentiError;
use crate::venti_lexer::token::Token;
use logos::Logos;

pub struct Lexer<'a> {
    lexer: logos::Lexer<'a, Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            lexer: Token::lexer(input),
        }
    }

    pub fn next_token(&mut self) -> Result<Token, VentiError> {
        match self.lexer.next() {
            Some(token) => Ok(token),
            None => Err(VentiError::SyntaxError(
                "Unexpected end of input".to_string(),
            )),
        }
    }
}
