use std::fmt::{Display, Error, Formatter};

#[derive(Clone, PartialEq, Debug)]
pub enum Punctuator {
    Assign,      // =
    Add,         // +
    Sub,         // -
    Mul,         // *
    Div,         // /
    LessThan,    // `<`
    GreaterThan, // `>`
    Eq,          // ==
    NotEq,       // !=
    Not,         // !

    Semicolon,    // ;
    Comma,        // ,
    OpenBlock,    // {
    CloseBlock,   // }
    OpenParen,    // (
    CloseParen,   // )
    Colon,        // :
    OpenBracket,  // [
    CloseBracket, // ]
}

impl Display for Punctuator {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{}",
            match self {
                Self::Add => "+",
                Self::Assign => "=",
                Self::Div => "/",
                Self::Eq => "==",
                Self::GreaterThan => ">",
                Self::LessThan => "<",
                Self::Mul => "*",
                Self::Not => "!",
                Self::OpenBlock => "{",
                Self::CloseBlock => "}",
                Self::OpenParen => "(",
                Self::CloseParen => ")",
                Self::Semicolon => ";",
                Self::Sub => "-",
                Self::Comma => ",",
                Self::NotEq => "!=",
                Self::Colon => ":",
                Self::OpenBracket => "[",
                Self::CloseBracket => "]",
            }
        )
    }
}
