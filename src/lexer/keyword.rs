use std::{fmt, str::FromStr};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Keyword {
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

#[derive(Debug, Clone, Copy)]
pub struct KeywordError;
impl fmt::Display for KeywordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid token")
    }
}

impl Keyword {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Function => "fn",
            Self::Let => "let",
            Self::True => "true",
            Self::False => "false",
            Self::If => "if",
            Self::Else => "else",
            Self::Return => "return",
        }
    }
}

impl FromStr for Keyword {
    type Err = KeywordError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fn" => Ok(Self::Function),
            "let" => Ok(Self::Let),
            "if" => Ok(Self::If),
            "else" => Ok(Self::Else),
            "true" => Ok(Self::True),
            "false" => Ok(Self::False),
            "return" => Ok(Self::Return),
            _ => Err(KeywordError),
        }
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}
