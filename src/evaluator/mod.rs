use core::fmt;
use std::rc::Rc;

use crate::{
    object::Object,
    parser::node::{Expression, Node, Program, Statement},
};

#[cfg(test)]
mod tests;

pub type EvalResult = Result<Rc<Object>, EvalError>;

#[derive(Debug)]
pub struct EvalError {
    pub message: String,
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub fn eval(node: &Node) -> EvalResult {
    match node {
        Node::Program(ref p) => eval_program(p),
        Node::Expression(ref exp) => eval_expression(exp),
        Node::Statement(ref stmt) => eval_statement(stmt),
    }
}

fn eval_program(p: &Program) -> EvalResult {
    let mut result = Rc::new(Object::Null);
    for stmt in &p.body {
        let res = eval_statement(stmt)?;
        result = res;
    }
    Ok(result)
}

fn eval_expression(exp: &Expression) -> EvalResult {
    match exp {
        Expression::Integer(num) => Ok(Rc::new(Object::Int(*num))),
        Expression::Boolean(b) => Ok(Rc::new(Object::Bool(*b))),
        _ => Err(EvalError {
            message: "unimplement eval expression".to_string(),
        }),
    }
}

fn eval_statement(stmt: &Statement) -> EvalResult {
    match stmt {
        Statement::Let(stmt) => {
            // TODO
            eval_expression(&stmt.value);
            todo!()
        }
        Statement::Return(ret) => {
            todo!()
        }
        Statement::Expression(exp) => eval_expression(&exp.expression),
    }
}
