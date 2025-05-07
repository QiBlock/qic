//! Token disambiguation
//!  - Longer beats shorter.
//!  - Specific beats generic.

use logos::Logos;
use num_derive::{FromPrimitive, ToPrimitive};

#[derive(
    Debug, Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Hash, FromPrimitive, ToPrimitive, Logos,
)]
pub enum TokenKind {
    /// The keyword lexeme.

    /// The `object` keyword.  priority: 12
    #[token("object")]
    Object,
    /// The `code` keyword.  priority: 8
    #[token("code")]
    Code,
    /// The `function` keyword.  priority: 16
    #[token("function")]
    Function,
    /// The `let` keyword.  priority: 6
    #[token("let")]
    Let,
    /// The `if` keyword.  priority: 4
    #[token("if")]
    If,
    /// The `switch` keyword.  priority: 12
    #[token("switch")]
    Switch,
    /// The `case` keyword.  priority: 8
    #[token("case")]
    Case,
    /// The `default` keyword.  priority: 14
    #[token("default")]
    Default,
    /// The `for` keyword.  priority: 6
    #[token("for")]
    For,
    /// The `break` keyword.  priority: 10
    #[token("break")]
    Break,
    /// The `continue` keyword.  priority: 16
    #[token("continue")]
    Continue,
    /// The `leave` keyword.  priority: 10
    #[token("leave")]
    Leave,
    /// The `bool` keyword.  priority: 8
    #[token("bool")]
    Bool,
    /// The `int{N}` keyword.  priority: 8
    #[regex(r"int[[:digit:]]+")]
    Int,
    /// The `uint{N}` keyword.  priority: 10
    #[regex(r"uint[[:digit:]]+")]
    Uint,

    /// The symbol lexeme.

    /// The `:=` symbol.  priority: 4
    #[token(":=")]
    Assignment,
    /// The `->` symbol.  priority: 4
    #[token("->")]
    Arrow,
    /// The `{` symbol.  priority: 2
    #[token("{")]
    BracketCurlyLeft,
    /// The `}` symbol.  priority: 2
    #[token("}")]
    BracketCurlyRight,
    /// The `(` symbol.  priority: 2
    #[token("(")]
    ParenthesisLeft,
    /// The `)` symbol.  priority: 2
    #[token(")")]
    ParenthesisRight,
    /// The `,` symbol.  priority: 2
    #[token(",")]
    Comma,
    /// The `:` symbol.  priority: 2
    #[token(":")]
    Colon,

    /// The literal lexeme.

    /// The `true` literal.  priority: 8
    #[token("true")]
    True,
    /// The `false` literal.  priority: 10
    #[token("false")]
    False,
    /// An integer literal, like `42`.  priority: 2
    #[regex(r"[[:digit:]]+")]
    Decimal,
    /// A hexadecimal literal, like `0xffff`.  priority: 6
    #[regex(r"0x[[:xdigit:]]+")]
    Hexadecimal,
    /// The string literal lexeme.  priority: 4
    #[regex(r#""([^"\r\n\\]|\\.)*""#)]
    String,
    /// The hex literal lexeme.  priority: 10
    #[regex(r#"hex("([[:xdigit:]]{2})*"|'([[:xdigit:]]{2})*')"#)]
    StringHex,

    /// The identifier lexeme.  priority: 2
    #[regex(r"[[:alpha:]_$][[:alnum:]_$.]*")]
    Identifier,

    /// The trivia lexeme.

    /// The comment lexeme.  priority: 4
    #[regex(r"//[^\n]*|/\*([^*]|\*[^/])*\*/")]
    Comment,
    /// The whitespace lexeme.  priority: 2
    #[regex(r"[[:space:]]+")]
    Whitespace,
}

impl TokenKind {
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::Object
                | Self::Code
                | Self::Function
                | Self::Let
                | Self::If
                | Self::Switch
                | Self::Case
                | Self::Default
                | Self::For
                | Self::Break
                | Self::Continue
                | Self::Leave
                | Self::Bool
                | Self::Int
                | Self::Uint
        )
    }
    pub fn is_symbol(&self) -> bool {
        matches!(
            self,
            Self::Assignment
                | Self::Arrow
                | Self::BracketCurlyLeft
                | Self::BracketCurlyRight
                | Self::ParenthesisLeft
                | Self::ParenthesisRight
                | Self::Comma
                | Self::Colon
        )
    }

    pub fn is_literal(&self) -> bool {
        matches!(
            self,
            Self::True
                | Self::False
                | Self::Decimal
                | Self::Hexadecimal
                | Self::String
                | Self::StringHex
        )
    }

    pub fn is_identifier(&self) -> bool {
        matches!(self, Self::Identifier)
    }

    pub fn is_trivia(&self) -> bool {
        matches!(self, Self::Comment | Self::Whitespace)
    }
}

#[cfg(test)]
mod tests {
    use logos_codegen::mir::Mir;
    use regex_syntax::escape;

    #[test]
    fn test_token_kind_priority() {
        let all = vec![
            Mir::utf8(&escape("object")),
            Mir::utf8(&escape("code")),
            Mir::utf8(&escape("function")),
            Mir::utf8(&escape("let")),
            Mir::utf8(&escape("if")),
            Mir::utf8(&escape("switch")),
            Mir::utf8(&escape("case")),
            Mir::utf8(&escape("default")),
            Mir::utf8(&escape("for")),
            Mir::utf8(&escape("break")),
            Mir::utf8(&escape("continue")),
            Mir::utf8(&escape("leave")),
            Mir::utf8(&escape("bool")),
            Mir::utf8(r"int[[:digit:]]+"),
            Mir::utf8(r"uint[[:digit:]]+"),
            Mir::utf8(&escape(":=")),
            Mir::utf8(&escape("->")),
            Mir::utf8(&escape("{")),
            Mir::utf8(&escape("}")),
            Mir::utf8(&escape("(")),
            Mir::utf8(&escape(")")),
            Mir::utf8(&escape(",")),
            Mir::utf8(&escape(":")),
            Mir::utf8(&escape("true")),
            Mir::utf8(&escape("false")),
            Mir::utf8(r"[[:digit:]]+"),
            Mir::utf8(r"0x[[:xdigit:]]+"),
            Mir::utf8(r#""([^"\r\n\\]|\\.)*""#),
            Mir::utf8(r#"hex("([[:xdigit:]]{2})*"|'([[:xdigit:]]{2})*')"#),
            Mir::utf8(r"[[:alpha:]_$][[:alnum:]_$.]*"),
            Mir::utf8(r"//[^\n]*|/\*([^*]|\*[^/])*\*/"),
            Mir::utf8(r"[[:space:]]+"),
        ];
        for mir in all {
            let res = mir.unwrap();
            println!("{:?}, priority: {}", res, res.priority());
        }
    }
}
