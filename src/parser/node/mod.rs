use std::fmt;

pub use self::{
    block_stmt::BlockStatement, call_expression::CallExpression,
    expression_stmt::ExpressionStatement, function_literal::*, hash_literal::HashLiteral,
    if_expression::IfExpression, infix_expression::InfixExpression, let_stmt::LetStatement,
    prefix_expression::PrefixExpression, return_stmt::ReturnStatement,
};

mod block_stmt;
mod call_expression;
mod expression_stmt;
mod function_literal;
mod hash_literal;
mod if_expression;
mod infix_expression;
mod let_stmt;
mod prefix_expression;
mod return_stmt;

#[derive(Debug)]
pub enum Node {
    Program(Box<Program>),
    Statement(Box<Statement>),
    Expression(Box<Expression>),
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Program(p) => write!(f, "{}", p),
            Node::Statement(stmt) => write!(f, "{}", stmt),
            Node::Expression(expr) => write!(f, "{}", expr),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Program {
    pub body: Vec<Statement>,
}

impl Program {
    pub fn new() -> Self {
        Program { body: Vec::new() }
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let statements: Vec<String> = (&self.body)
            .into_iter()
            .map(|stmt| stmt.to_string())
            .collect();
        write!(f, "{}", statements.join(""))
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Statement {
    Let(Box<LetStatement>),
    Return(Box<ReturnStatement>),
    Expression(Box<ExpressionStatement>),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Statement::Let(stmt) => format!("{}", stmt),
                Statement::Return(ret) => format!("{}", ret),
                Statement::Expression(exp) => format!("{}", exp),
            }
        )
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Expression {
    Integer(i64),
    Identifier(String),
    String(String),
    Boolean(bool),
    Prefix(Box<PrefixExpression>),
    Infix(Box<InfixExpression>),
    If(Box<IfExpression>),
    Function(Box<FunctionLiteral>),
    Call(Box<CallExpression>),
    Array(Box<ArrayLiteral>),
    Index(Box<IndexExpression>),
    Hash(Box<HashLiteral>),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Expression::Integer(value) => format!("{}", value),
                Expression::Identifier(value) => format!("{}", value),
                Expression::String(value) => format!("{}", value),
                Expression::Boolean(value) => format!("{}", value),
                Expression::Prefix(value) => format!("{}", value),
                Expression::Infix(value) => format!("{}", value),
                Expression::If(value) => format!("{}", value),
                Expression::Function(value) => format!("{}", value),
                Expression::Call(value) => format!("{}", value),
                Expression::Array(a) => format!("{}", a),
                Expression::Index(i) => format!("{}", i),
                Expression::Hash(h) => format!("{}", h),
            }
        )
    }
}

impl From<bool> for Expression {
    #[inline]
    fn from(n: bool) -> Self {
        Self::Boolean(n)
    }
}

impl From<i64> for Expression {
    #[inline]
    fn from(n: i64) -> Self {
        Self::Integer(n)
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct ArrayLiteral {
    pub elements: Vec<Expression>,
}

impl fmt::Display for ArrayLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let elements: Vec<String> = (&self.elements)
            .into_iter()
            .map(|e| e.to_string())
            .collect();
        write!(f, "[{}]", elements.join(", "))
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct IndexExpression {
    pub left: Expression,
    pub index: Expression,
}

impl fmt::Display for IndexExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}[{}])", self.left.to_string(), self.index.to_string())
    }
}
