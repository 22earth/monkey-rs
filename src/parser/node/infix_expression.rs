use std::fmt;

use crate::lexer::token::TokenKind;

use super::Expression;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct InfixExpression {
    pub operator: TokenKind,
    pub left: Expression,
    pub right: Expression,
}

impl fmt::Display for InfixExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.left, self.operator, self.right)
    }
}
