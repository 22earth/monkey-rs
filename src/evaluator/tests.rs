use std::{cell::RefCell, rc::Rc};

use super::*;
use crate::{object::Object, parser};

fn test_eval(input: &str) -> Rc<Object> {
    match parser::parse(input) {
        Ok(node) => eval(&node).expect(input),
        Err(e) => panic!("error {:?} on input {}", e, input),
    }
}

fn test_integer_object(obj: &Object, expected: i64) {
    match obj {
        Object::Int(i) => assert_eq!(i, &expected),
        _ => panic!("expected integer object, but got {:?}", obj),
    }
}

fn test_bool_object(obj: &Object, expected: bool) {
    match obj {
        Object::Bool(i) => assert_eq!(i, &expected),
        _ => panic!("expected bool object, but got {:?}", obj),
    }
}

fn test_null_object(obj: &Object) {
    match obj {
        Object::Null => {}
        _ => panic!("expected bool object, but got {:?}", obj),
    }
}
#[test]
fn eval_integer_expression() {
    let tests = vec![("5", 5), ("10", 10)];

    for t in tests {
        let evaluated = test_eval(t.0);
        test_integer_object(&evaluated, t.1);
    }
}

#[test]
fn eval_boolean_expression() {
    let tests = vec![("true", true), ("false", false)];

    for t in tests {
        let evaluated = test_eval(t.0);
        test_bool_object(&evaluated, t.1);
    }
}
