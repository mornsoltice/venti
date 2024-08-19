use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[regex(r"[ \t\n\f]+", logos::skip)]
    //#[error]
    Error,

    #[token("venti")]
    Venti,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),

    #[regex(r#""[^"]*""#, |lex| lex.slice().to_string())]
    StringLiteral(String),

    #[regex(r"[0-9]+", |lex| lex.slice().to_string())]
    NumberLiteral(i64),

    #[regex(r"true|false", |lex| lex.slice().to_string())]
    BooleanLiteral(bool),

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
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
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
    #[token("async")]
    Async,
    #[token("await")]
    Await,

    #[token("int")]
    Int,
    #[token("float")]
    Float,
    #[token("bool")]
    Bool,
}

