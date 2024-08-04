mod codegen;
mod errors;
mod venti_lexer;
mod venti_parser;

use crate::errors::VentiError;
use clap::{Arg, Command};
use std::fs;

fn main() -> Result<(), VentiError> {
    let matches = Command::new("Venti")
        .version("0.1.0")
        .author("k m nandyka")
        .about("simple programming language with rust, i made it because i love venti")
        .arg(
            Arg::new("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    let input = matches.get_one::<String>("INPUT").unwrap();
    let source = fs::read_to_string(input).expect("Could not read file");

    let mut lexer = venti_lexer::lexer::Lexer::new(&source);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token()? {
        tokens.push(token);
    }
    println!("Tokens: {:?}", tokens);

    let mut parser = venti_parser::parser::Parser::new(tokens);
    let ast = parser.parse()?;
    println!("AST: {:?}", ast);

    let context = inkwell::context::Context::create();
    let codegen = codegen::CodeGen::new(&context);
    codegen.compile(ast)?;

    Ok(())
}
