use std::fmt;

use crate::lexer::token::TokenKind;

use super::Expression;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct PrefixExpression {
    pub operator: TokenKind,
    pub right: Expression,
}

impl fmt::Display for PrefixExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}{})", self.operator, self.right)
    }
}
