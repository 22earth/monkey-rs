use std::fmt;

use super::BlockStatement;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct FunctionLiteral {
    pub parameters: Vec<IdentifierExpression>,
    pub body: BlockStatement,
}

impl fmt::Display for FunctionLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let param_list: Vec<String> = (&self.parameters)
            .into_iter()
            .map(|p| p.to_string())
            .collect();
        write!(f, "({}) {}", param_list.join(", "), self.body)
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct IdentifierExpression {
    pub name: String,
}

impl fmt::Display for IdentifierExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
