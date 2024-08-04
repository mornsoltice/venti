use std::fmt;

#[derive(Debug)]
pub enum VentiError {
    SyntaxError(String),
    TypeError(String),
    RuntimeError(String),
    CodegenError(String),
    IOError(String),
}

impl fmt::Display for VentiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VentiError::SyntaxError(ref msg) => write!(f, "Syntax Error: {}", msg),
            VentiError::TypeError(ref msg) => write!(f, "Type Error: {}", msg),
            VentiError::RuntimeError(ref msg) => write!(f, "Runtime Error: {}", msg),
            VentiError::CodegenError(ref msg) => write!(f, "Codegen Error: {}", msg),
            VentiError::IOError(ref msg) => write!(f, "IO Error: {}", msg),
        }
    }
}

impl std::error::Error for VentiError {}
