use core::fmt;
use std::rc::Rc;

use crate::{
    lexer::{punctuator::Punctuator, token::TokenKind},
    object::{self, Object},
    parser::node::{BlockStatement, Expression, Node, Program, Statement},
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
        // 处理 return
        match &*res {
            Object::Return(r) => return Ok(Rc::clone(&r.value)),
            _ => result = res,
        }
    }
    Ok(result)
}

fn eval_expression(exp: &Expression) -> EvalResult {
    match exp {
        Expression::Integer(num) => Ok(Rc::new(Object::Int(*num))),
        Expression::Boolean(b) => Ok(Rc::new(Object::Bool(*b))),
        Expression::Prefix(exp) => {
            let right = eval_expression(&exp.right)?;
            eval_prefix_expression(&exp.operator, right)
        }
        Expression::Infix(exp) => {
            let left = eval_expression(&exp.left)?;
            let right = eval_expression(&exp.right)?;
            eval_infix_expression(&exp.operator, left, right)
        }
        Expression::If(if_exp) => {
            let evaluated = eval_expression(&if_exp.condition)?;
            match is_truthy(&evaluated) {
                true => eval_block_statement(&if_exp.consequence),
                false => match &if_exp.alternative {
                    Some(alt) => eval_block_statement(alt),
                    None => Ok(Rc::new(Object::Null)),
                },
            }
        }
        _ => Err(EvalError {
            message: "unimplement eval expression".to_string(),
        }),
    }
}

fn eval_block_statement(consequence: &BlockStatement) -> Result<Rc<Object>, EvalError> {
    let mut result = Rc::new(Object::Null);

    for stmt in &consequence.statements {
        let res = eval_statement(stmt)?;
        match *res {
            Object::Return(_) => return Ok(res),
            _ => result = res,
        }
    }

    Ok(result)
}
fn is_truthy(obj: &Object) -> bool {
    match obj {
        Object::Null => false,
        Object::Bool(false) => false,
        _ => true,
    }
}

fn eval_infix_expression(
    operator: &TokenKind,
    left: Rc<Object>,
    right: Rc<Object>,
) -> Result<Rc<Object>, EvalError> {
    match (&*left, &*right) {
        (Object::Int(l), Object::Int(r)) => eval_integer_infix_expression(operator, *l, *r),
        (Object::Bool(l), Object::Bool(r)) => eval_bool_infix_expression(operator, *l, *r),
        _ => Err(EvalError {
            message: format!("type mismatch: {:?} {} {:?}", left, operator, right),
        }),
    }
}

fn eval_bool_infix_expression(
    operator: &TokenKind,
    l: bool,
    r: bool,
) -> Result<Rc<Object>, EvalError> {
    match operator {
        TokenKind::Punctuator(Punctuator::Eq) => Ok(Rc::new(Object::Bool(l == r))),
        TokenKind::Punctuator(Punctuator::NotEq) => Ok(Rc::new(Object::Bool(l != r))),
        _ => {
            return Err(EvalError {
                message: format!("unknown operator {}", operator),
            })
        }
    }
}

fn eval_integer_infix_expression(
    operator: &TokenKind,
    l: i64,
    r: i64,
) -> Result<Rc<Object>, EvalError> {
    match operator {
        TokenKind::Punctuator(Punctuator::Add) => Ok(Rc::new(Object::Int(l + r))),
        TokenKind::Punctuator(Punctuator::Sub) => Ok(Rc::new(Object::Int(l - r))),
        TokenKind::Punctuator(Punctuator::Mul) => Ok(Rc::new(Object::Int(l * r))),
        TokenKind::Punctuator(Punctuator::Div) => Ok(Rc::new(Object::Int(l / r))),
        TokenKind::Punctuator(Punctuator::GreaterThan) => Ok(Rc::new(Object::Bool(l > r))),
        TokenKind::Punctuator(Punctuator::LessThan) => Ok(Rc::new(Object::Bool(l < r))),
        TokenKind::Punctuator(Punctuator::Eq) => Ok(Rc::new(Object::Bool(l == r))),
        TokenKind::Punctuator(Punctuator::NotEq) => Ok(Rc::new(Object::Bool(l != r))),
        _ => {
            return Err(EvalError {
                message: format!("unknown operator {}", operator),
            })
        }
    }
}

fn eval_prefix_expression(
    operator: &TokenKind,
    right: Rc<Object>,
) -> Result<Rc<Object>, EvalError> {
    match operator {
        TokenKind::Punctuator(Punctuator::Not) => eval_bang_operator_expression(right),
        TokenKind::Punctuator(Punctuator::Sub) => eval_minus_prefix_operator_expression(right),
        _ => Err(EvalError {
            message: format!("unknown prefix operator {}", operator),
        }),
    }
}

fn eval_minus_prefix_operator_expression(right: Rc<Object>) -> Result<Rc<Object>, EvalError> {
    match *right {
        Object::Int(num) => Ok(Rc::new(Object::Int(-num))),
        _ => Err(EvalError {
            message: format!("unknown operator: -{:?}", right),
        }),
    }
}

fn eval_bang_operator_expression(right: Rc<Object>) -> Result<Rc<Object>, EvalError> {
    Ok(Rc::new(match *right {
        Object::Bool(true) => Object::Bool(false),
        Object::Bool(false) => Object::Bool(true),
        Object::Null => Object::Bool(true),
        Object::Int(0) => Object::Bool(true),
        Object::Int(_) => Object::Bool(false),
        _ => Object::Bool(false),
    }))
}

fn eval_statement(stmt: &Statement) -> EvalResult {
    match stmt {
        Statement::Let(stmt) => {
            // TODO
            eval_expression(&stmt.value);
            todo!()
        }
        Statement::Return(ret) => {
            let value = eval_expression(&ret.value)?;
            Ok(Rc::new(Object::Return(Rc::new(object::Return { value }))))
        }
        Statement::Expression(exp) => eval_expression(&exp.expression),
    }
}
