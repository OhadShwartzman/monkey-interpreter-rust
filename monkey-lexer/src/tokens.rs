use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum TokenTypes {
    Illegal,
    Eof,

    Ident,
    Int,

    Assign,
    Plus,

    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    Function,
    Let
}

impl FromStr for TokenTypes {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            _ => Err(())
        }
    }
}
