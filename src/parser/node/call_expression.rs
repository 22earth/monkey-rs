use std::fmt;

use super::Expression;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct CallExpression {
    pub function: Expression,
    pub arguments: Vec<Expression>,
}

impl fmt::Display for CallExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arg_list: Vec<String> = (&self.arguments)
            .into_iter()
            .map(|exp| exp.to_string())
            .collect();
        write!(f, "{}({})", self.function.to_string(), arg_list.join(", "))
    }
}
