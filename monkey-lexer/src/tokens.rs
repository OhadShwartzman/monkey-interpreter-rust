use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum TokenTypes {
    Illegal,
    Eof,

    Ident(String),
    Int(i64),

    Assign,
    Plus,

    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    Function,
    Let,
}

impl FromStr for TokenTypes {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        Ok(match string {
            "" => Self::Eof,
            "=" => Self::Assign,
            "+" => Self::Plus,
            "(" => Self::LParen,
            ")" => Self::RParen,
            "{" => Self::LBrace,
            "}" => Self::RBrace,
            "," => Self::Comma,
            ";" => Self::Semicolon,

            _ => Self::Illegal,
        })
    }
}
