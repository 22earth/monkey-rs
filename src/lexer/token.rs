use super::{keyword::Keyword, punctuator::Punctuator};
use std::fmt;

pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    /// Create a new detailed token from the token data, line number and column number
    #[inline]
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    /// Gets the kind of the token.
    #[inline]
    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }

    /// Gets the token span in the original source code.
    #[inline]
    pub fn span(&self) -> Span {
        self.span
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Numeric {
    // An integer
    Integer(i64),
}

impl From<i64> for Numeric {
    #[inline]
    fn from(n: i64) -> Self {
        Self::Integer(n)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum TokenKind {
    BooleanLiteral(bool),
    EOF,
    Identifier(Box<str>),
    Keyword(Keyword),
    Punctuator(Punctuator),
    NumericLiteral(Numeric),
    StringLiteral(Box<str>),
    // TODO comment
    // Comment
    // TODO user error
    ILLEGAL,
}

impl From<bool> for TokenKind {
    fn from(oth: bool) -> Self {
        Self::BooleanLiteral(oth)
    }
}

impl From<Keyword> for TokenKind {
    fn from(kw: Keyword) -> Self {
        Self::Keyword(kw)
    }
}

impl From<Punctuator> for TokenKind {
    fn from(punc: Punctuator) -> Self {
        Self::Punctuator(punc)
    }
}

impl From<Numeric> for TokenKind {
    fn from(num: Numeric) -> Self {
        Self::NumericLiteral(num)
    }
}

impl TokenKind {
    /// Creates a `BooleanLiteral` token kind.
    pub fn boolean_literal(lit: bool) -> Self {
        Self::BooleanLiteral(lit)
    }

    /// Creates an `EOF` token kind.
    pub fn eof() -> Self {
        Self::EOF
    }

    /// Creates an `Identifier` token type.
    pub fn identifier<I>(ident: I) -> Self
    where
        I: Into<Box<str>>,
    {
        Self::Identifier(ident.into())
    }

    /// Creates a `Keyword` token kind.
    pub fn keyword(keyword: Keyword) -> Self {
        Self::Keyword(keyword)
    }

    /// Creates a `NumericLiteral` token kind.
    pub fn numeric_literal<L>(lit: L) -> Self
    where
        L: Into<Numeric>,
    {
        Self::NumericLiteral(lit.into())
    }

    /// Creates a `Punctuator` token type.
    pub fn punctuator(punc: Punctuator) -> Self {
        Self::Punctuator(punc)
    }

    /// Creates a `StringLiteral` token type.
    pub fn string_literal<S>(lit: S) -> Self
    where
        S: Into<Box<str>>,
    {
        Self::StringLiteral(lit.into())
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::BooleanLiteral(ref val) => write!(f, "{}", val),
            Self::EOF => write!(f, "end of file"),
            Self::Identifier(ref ident) => write!(f, "{}", ident),
            Self::Keyword(ref word) => write!(f, "{}", word),
            Self::NumericLiteral(Numeric::Integer(num)) => write!(f, "{}", num),
            Self::StringLiteral(ref lit) => write!(f, "{}", lit),
            Self::Punctuator(ref punc) => write!(f, "{}", punc),
            Self::ILLEGAL => write!(f, "ILLEGAL"),
        }
    }
}

pub fn lookup_identifier(identifier: &str) -> TokenKind {
    match identifier {
        "fn" => Keyword::Function.into(),
        "let" => Keyword::Let.into(),
        "true" => Keyword::True.into(),
        "false" => Keyword::False.into(),
        "if" => Keyword::If.into(),
        "else" => Keyword::Else.into(),
        "return" => Keyword::Return.into(),
        _ => TokenKind::identifier(identifier),
    }
}
