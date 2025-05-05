pub mod token_kind;

use logos::Logos;
use rowan::{TextRange, TextSize};
use std::ops::Range as StdRange;
pub use token_kind::TokenKind;

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub kind: Result<TokenKind, ()>,
    pub text: &'a str,
    pub range: TextRange,
}

pub struct Lexer<'a> {
    inner: logos::Lexer<'a, TokenKind>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let inner = TokenKind::lexer(input);
        Self { inner }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.inner.next()?;
        let text = self.inner.slice();

        let range = {
            let StdRange { start, end } = self.inner.span();
            let start = TextSize::try_from(start).unwrap();
            let end = TextSize::try_from(end).unwrap();

            TextRange::new(start, end)
        };

        Some(Self::Item { kind, text, range })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{BufReader, Read};

    #[test]
    fn test_tokenize() {
        let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let file = File::open(format!("{}/../../test/ERC20.sol.ERC20.yul", crate_dir)).unwrap();
        let mut reader = BufReader::new(file);
        let mut source = String::new();
        reader.read_to_string(&mut source).unwrap();

        let mut lexer = Lexer::new(source.as_str());
        while let Some(token) = lexer.next() {
            if token.kind.is_err() {
                panic!("Error tokenizing: {:?}", token);
            }
            let token_kind = token.kind.unwrap();
            if token_kind.is_skip() {
                continue;
            }
            println!("Token: {:?}", token);
        }
    }
}
