use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[regex(r"[ \t\n\f]+", logos::skip)]
    #[error]
    Error,

    #[token("venti")]
    Venti,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    #[regex(r#""[^"]*""#)]
    StringLiteral,

    #[regex(r"[0-9]+")]
    NumberLiteral,

    #[regex(r"true|false")]
    BooleanLiteral,

    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[token("=")]
    Equals,
    #[token("if_venti")]
    If,
    #[token("else_venti")]
    Else,
    #[token("for_venti")]
    For,
    #[token("while_venti")]
    While,
    #[token("printventi")]
    Print,
}
