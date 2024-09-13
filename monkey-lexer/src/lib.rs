pub mod tokens;

use std::{iter::Peekable, str::Chars, str::FromStr};

use tokens::TokenTypes;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
        }
    }

    pub fn get_next_token(&mut self) -> Option<tokens::TokenTypes> {
        let mut next;

        loop {
            next = self.input.next()?;

            if next != ' ' {
                break;
            }
        }

        while let TokenTypes::Ident(identifier_part) =
            TokenTypes::from_str(&next.to_string()).ok()?
        {
            next = self.input.next()?;
        }

        tokens::TokenTypes::from_str(&next.to_string()).ok()
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = tokens::TokenTypes;

    fn next(&mut self) -> Option<Self::Item> {
        self.get_next_token()
    }
}

#[cfg(test)]
mod test {
    use super::{tokens::TokenTypes, Lexer};

    #[test]
    fn test_basic_token_stream() {
        const STREAM: &'static str = "{ +( ) }";

        assert_eq!(
            Lexer::new(STREAM).collect::<Vec<TokenTypes>>(),
            [
                TokenTypes::LBrace,
                TokenTypes::Plus,
                TokenTypes::LParen,
                TokenTypes::RParen,
                TokenTypes::RBrace
            ]
        );
    }

    #[test]
    fn test_advanced_token_stream() {
        const STREAM: &'static str = "{ a = 5 }";

        assert_eq!(
            Lexer::new(STREAM).collect::<Vec<TokenTypes>>(),
            [
                TokenTypes::LBrace,
                TokenTypes::Ident("a".to_string()),
                TokenTypes::Assign,
                TokenTypes::Int(5),
                TokenTypes::RBrace
            ]
        );
    }
}
