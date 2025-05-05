use logos::Logos;
use num_derive::{FromPrimitive, ToPrimitive};

#[derive(
    Debug, Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Hash, FromPrimitive, ToPrimitive, Logos,
)]
pub enum TokenKind {
    /// The keyword lexeme.

    /// The `object` keyword.
    #[token("object")]
    Object,
    /// The `code` keyword.
    #[token("code")]
    Code,
    /// The `function` keyword.
    #[token("function")]
    Function,
    /// The `let` keyword.
    #[token("let")]
    Let,
    /// The `if` keyword.
    #[token("if")]
    If,
    /// The `switch` keyword.
    #[token("switch")]
    Switch,
    /// The `case` keyword.
    #[token("case")]
    Case,
    /// The `default` keyword.
    #[token("default")]
    Default,
    /// The `for` keyword.
    #[token("for")]
    For,
    /// The `break` keyword.
    #[token("break")]
    Break,
    /// The `continue` keyword.
    #[token("continue")]
    Continue,
    /// The `leave` keyword.
    #[token("leave")]
    Leave,
    /// The `bool` keyword.
    #[token("bool")]
    Bool,
    /// The `int{N}` keyword.
    #[regex(r"int[[:digit:]]+")]
    Int,
    /// The `uint{N}` keyword.
    #[regex(r"uint[[:digit:]]+")]
    Uint,

    /// The symbol lexeme.

    /// The `:=` symbol.
    #[token(":=")]
    Assignment,
    /// The `->` symbol.
    #[token("->")]
    Arrow,
    /// The `{` symbol.
    #[token("{")]
    BracketCurlyLeft,
    /// The `}` symbol.
    #[token("}")]
    BracketCurlyRight,
    /// The `(` symbol.
    #[token("(")]
    ParenthesisLeft,
    /// The `)` symbol.
    #[token(")")]
    ParenthesisRight,
    /// The `,` symbol.
    #[token(",")]
    Comma,
    /// The `:` symbol.
    #[token(":")]
    Colon,

    /// The identifier lexeme.

    #[regex(r"[[:alpha:]_$][[:alnum:]_$.]*")]
    Identifier,

    /// The literal lexeme.

    /// The `true` literal.
    #[token("true")]
    True,
    /// The `false` literal.
    #[token("false")]
    False,
    /// An integer literal, like `42`.
    #[regex(r"[[:digit:]]+")]
    Decimal,
    /// A hexadecimal literal, like `0xffff`.
    #[regex(r"0x[[:xdigit:]]+")]
    Hexadecimal,
    /// The string literal lexeme.
    #[regex(r#""([^"\r\n\\]|\\.)*""#)]
    String,
    /// The hex literal lexeme.
    #[regex(r#"hex("([[:xdigit:]]{2})*"|'([[:xdigit:]]{2})*')"#)]
    StringHex,

    /// The skip lexeme.

    /// The comment lexeme.
    #[regex(r"//[^\n]*")]
    Comment,
    /// The block comment lexeme.
    #[regex(r"/\*([^*]|\*[^/])*\*/")]
    BlockComment,
    /// The whitespace lexeme.
    #[regex(r"[[:space:]]+")]
    Whitespace,
}

impl TokenKind {
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            TokenKind::Object
                | TokenKind::Code
                | TokenKind::Function
                | TokenKind::Let
                | TokenKind::If
                | TokenKind::Switch
                | TokenKind::Case
                | TokenKind::Default
                | TokenKind::For
                | TokenKind::Break
                | TokenKind::Continue
                | TokenKind::Leave
                | TokenKind::Bool
                | TokenKind::Int
                | TokenKind::Uint
        )
    }
    pub fn is_symbol(&self) -> bool {
        matches!(
            self,
            TokenKind::Assignment
                | TokenKind::Arrow
                | TokenKind::BracketCurlyLeft
                | TokenKind::BracketCurlyRight
                | TokenKind::ParenthesisLeft
                | TokenKind::ParenthesisRight
                | TokenKind::Comma
                | TokenKind::Colon
        )
    }

    pub fn is_identifier(&self) -> bool {
        matches!(self, TokenKind::Identifier)
    }

    pub fn is_literal(&self) -> bool {
        matches!(
            self,
            TokenKind::True
                | TokenKind::False
                | TokenKind::Decimal
                | TokenKind::Hexadecimal
                | TokenKind::String
                | TokenKind::StringHex
        )
    }

    pub fn is_skip(&self) -> bool {
        matches!(
            self,
            TokenKind::Comment | TokenKind::BlockComment | TokenKind::Whitespace
        )
    }
}
