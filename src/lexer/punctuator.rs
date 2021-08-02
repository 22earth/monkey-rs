use std::{convert::TryInto, fmt};

use super::op::{BinOp, CompOp, LogOp, NumOp};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Copy)]
pub enum Punctuator {
    Assign,          // =
    Add,             // +
    Sub,             // -
    Mul,             // *
    Div,             // /
    LessThan,        // `<`
    LessThanOrEq,    // <=
    GreaterThan,     // `>`
    GreaterThanOrEq, // >=
    Eq,              // ==
    NotEq,           // !=
    Not,             // !

    Semicolon,    // ;
    Comma,        // ,
    OpenBlock,    // {
    CloseBlock,   // }
    OpenParen,    // (
    CloseParen,   // )
    Colon,        // :
    OpenBracket,  // [
    CloseBracket, // ]

    /// `&&`
    BoolAnd,
    /// `||`
    BoolOr,
}

impl Punctuator {
    // ?? &self or self
    pub fn as_binop(self) -> Option<BinOp> {
        match self {
            Self::Add => Some(BinOp::Num(NumOp::Add)),
            Self::Sub => Some(BinOp::Num(NumOp::Sub)),
            Self::Mul => Some(BinOp::Num(NumOp::Mul)),
            Self::Div => Some(BinOp::Num(NumOp::Div)),
            Self::BoolAnd => Some(BinOp::Log(LogOp::And)),
            Self::BoolOr => Some(BinOp::Log(LogOp::Or)),
            Self::Eq => Some(BinOp::Comp(CompOp::Equal)),
            Self::NotEq => Some(BinOp::Comp(CompOp::NotEqual)),
            Self::LessThan => Some(BinOp::Comp(CompOp::LessThan)),
            Self::GreaterThan => Some(BinOp::Comp(CompOp::GreaterThan)),
            Self::GreaterThanOrEq => Some(BinOp::Comp(CompOp::GreaterThanOrEqual)),
            Self::LessThanOrEq => Some(BinOp::Comp(CompOp::LessThanOrEqual)),
            _ => None,
        }
    }
}

impl TryInto<BinOp> for Punctuator {
    type Error = String;

    fn try_into(self) -> Result<BinOp, Self::Error> {
        self.as_binop()
            .ok_or_else(|| format!("No binary operator for {}", self))
    }
}

impl fmt::Display for Punctuator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Add => "+",
                Self::Assign => "=",
                Self::Div => "/",
                Self::Eq => "==",
                Self::GreaterThan => ">",
                Self::GreaterThanOrEq => ">=",
                Self::LessThan => "<",
                Self::LessThanOrEq => "<=",
                Self::Mul => "*",
                Self::Not => "!",
                Self::OpenBlock => "{{",
                Self::CloseBlock => "}}",
                Self::OpenParen => "(",
                Self::CloseParen => ")",
                Self::Semicolon => ";",
                Self::Sub => "-",
                Self::Comma => ",",
                Self::NotEq => "!=",
                Self::Colon => ":",
                Self::OpenBracket => "[",
                Self::CloseBracket => "]",
                Self::BoolAnd => "&&",
                Self::BoolOr => "||",
            }
        )
    }
}
