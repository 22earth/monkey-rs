use core::fmt;
use std::{cell::RefCell, rc::Rc};

use crate::{
    lexer::{punctuator::Punctuator, token::TokenKind},
    object::{self, Environment, Function, Object},
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

pub fn eval(node: &Node, env: Rc<RefCell<Environment>>) -> EvalResult {
    match node {
        Node::Program(ref p) => eval_program(p, env),
        Node::Expression(ref exp) => eval_expression(exp, env),
        Node::Statement(ref stmt) => eval_statement(stmt, env),
    }
}

fn eval_program(p: &Program, env: Rc<RefCell<Environment>>) -> EvalResult {
    let mut result = Rc::new(Object::Null);
    for stmt in &p.body {
        let res = eval_statement(stmt, Rc::clone(&env))?;
        // 处理 return
        match &*res {
            Object::Return(r) => return Ok(Rc::clone(&r.value)),
            _ => result = res,
        }
    }
    Ok(result)
}

fn eval_expression(exp: &Expression, env: Rc<RefCell<Environment>>) -> EvalResult {
    match exp {
        Expression::Integer(num) => Ok(Rc::new(Object::Int(*num))),
        Expression::Boolean(b) => Ok(Rc::new(Object::Bool(*b))),
        Expression::Prefix(exp) => {
            let right = eval_expression(&exp.right, env)?;
            eval_prefix_expression(&exp.operator, right)
        }
        Expression::Infix(exp) => {
            let left = eval_expression(&exp.left, Rc::clone(&env))?;
            let right = eval_expression(&exp.right, env)?;
            eval_infix_expression(&exp.operator, left, right)
        }
        Expression::If(if_exp) => {
            let evaluated = eval_expression(&if_exp.condition, Rc::clone(&env))?;
            match is_truthy(&evaluated) {
                true => eval_block_statement(&if_exp.consequence, env),
                false => match &if_exp.alternative {
                    Some(alt) => eval_block_statement(alt, env),
                    None => Ok(Rc::new(Object::Null)),
                },
            }
        }
        Expression::Identifier(ident) => eval_identifier(ident, env),
        Expression::Function(f) => {
            let func = Function {
                parameters: f.parameters.clone(),
                body: f.body.clone(),
                env: Rc::clone(&env),
            };
            Ok(Rc::new(Object::Function(Rc::new(func))))
        }
        Expression::Call(exp) => {
            let function = eval_expression(&exp.function, Rc::clone(&env))?;
            let args = eval_expressions(&exp.arguments, env)?;
            apply_function(&function, &args)
        }
        _ => Err(EvalError {
            message: "unimplement eval expression".to_string(),
        }),
    }
}

fn eval_identifier(ident: &str, env: Rc<RefCell<Environment>>) -> Result<Rc<Object>, EvalError> {
    match env.borrow().get(ident) {
        Some(obj) => Ok(obj),
        None => Err(EvalError {
            message: format!("identifier not found: {}", ident),
        }),
    }
}

fn eval_block_statement(
    consequence: &BlockStatement,
    env: Rc<RefCell<Environment>>,
) -> Result<Rc<Object>, EvalError> {
    let mut result = Rc::new(Object::Null);

    for stmt in &consequence.statements {
        let res = eval_statement(stmt, Rc::clone(&env))?;
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
                message: format!("unknown operator: {} {} {}", l, operator, r),
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
                message: format!("unknown operator: {} {} {}", l, operator, r),
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

fn eval_statement(stmt: &Statement, env: Rc<RefCell<Environment>>) -> EvalResult {
    match stmt {
        Statement::Let(stmt) => {
            let exp = eval_expression(&stmt.value, Rc::clone(&env))?;
            let obj = Rc::clone(&exp);
            env.borrow_mut().set(stmt.name.clone(), obj);
            Ok(exp)
        }
        Statement::Return(ret) => {
            let value = eval_expression(&ret.value, env)?;
            Ok(Rc::new(Object::Return(Rc::new(object::Return { value }))))
        }
        Statement::Expression(exp) => eval_expression(&exp.expression, env),
    }
}

fn eval_expressions(
    exps: &Vec<Expression>,
    env: Rc<RefCell<Environment>>,
) -> Result<Vec<Rc<Object>>, EvalError> {
    let mut objs = Vec::with_capacity(exps.len());

    for e in exps {
        let res = eval_expression(&e, Rc::clone(&env))?;
        objs.push(res);
    }

    Ok(objs)
}

fn apply_function(func: &Object, args: &Vec<Rc<Object>>) -> EvalResult {
    match func {
        Object::Function(f) => {
            let extended_env = extend_function_env(f, args);
            let evaluated = eval_block_statement(&f.body, extended_env)?;
            Ok(unwrap_return_value(evaluated))
        }
        // Object::Builtin(b) => match b.apply(args) {
        //     Ok(obj) => Ok(obj),
        //     Err(err) => Err(EvalError { message: err }),
        // },
        f => Err(EvalError {
            message: format!("{:?} is not a function", f),
        }),
    }
}

fn extend_function_env(func: &Function, args: &Vec<Rc<Object>>) -> Rc<RefCell<Environment>> {
    let env = Rc::new(RefCell::new(Environment::new_enclosed(Rc::clone(
        &func.env,
    ))));

    let mut args_iter = args.into_iter();

    for param in &func.parameters {
        let arg = args_iter.next().unwrap();

        env.borrow_mut().set(param.name.clone(), Rc::clone(arg))
    }

    env
}

fn unwrap_return_value(obj: Rc<Object>) -> Rc<Object> {
    if let Object::Return(ret) = &*obj {
        return Rc::clone(&ret.value);
    }
    obj
}
