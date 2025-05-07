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
