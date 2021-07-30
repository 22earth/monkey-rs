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
    let tests = [
        ("5", 5),
        ("10", 10),
        ("-5", -5),
        ("-10", -10),
        ("5 + 5 + 5 + 5 - 10", 10),
        ("2 * 2 * 2 * 2 * 2", 32),
        ("-50 + 100 + -50", 0),
        ("5 * 2 + 10", 20),
        ("5 + 2 * 10", 25),
        ("20 + 2 * -10", 0),
        ("50 / 2 * 2 + 10", 60),
        ("2 * (5 + 10)", 30),
        ("3 * 3 * 3 + 10", 37),
        ("3 * (3 * 3) + 10", 37),
        ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
    ];

    for t in tests {
        let evaluated = test_eval(t.0);
        test_integer_object(&evaluated, t.1);
    }
}

#[test]
fn eval_boolean_expression() {
    let tests = [
        ("true", true),
        ("false", false),
        ("1 < 2", true),
        ("1 > 2", false),
        ("1 < 1", false),
        ("1 > 1", false),
        ("1 == 1", true),
        ("1 != 1", false),
        ("1 == 2", false),
        ("1 != 2", true),
        ("true == true", true),
        ("false == false", true),
        ("true == false", false),
        ("true != false", true),
        ("false != true", true),
        ("(1 < 2) == true", true),
        ("(1 < 2) == false", false),
        ("(1 > 2) == true", false),
        ("(1 > 2) == false", true),
    ];

    for t in tests {
        let evaluated = test_eval(t.0);
        test_bool_object(&evaluated, t.1);
    }
}

#[test]
fn test_bang_operator() {
    let tests = [
        ("!true", false),
        ("!false", true),
        ("!5", false),
        ("!!true", true),
        ("!!false", false),
        ("!!5", true),
        ("!!0", false),
    ];
    for t in tests {
        let evaluated = test_eval(t.0);
        test_bool_object(&evaluated, t.1);
    }
}
