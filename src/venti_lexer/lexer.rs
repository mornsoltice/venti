use logos::Logos;
use crate::venti_lexer::token::Token;

pub struct Lexer<'a> {
    lexer: logos::Lexer<'a, Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            lexer: Token::lexer(input),
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.lexer.next()
    }
}
