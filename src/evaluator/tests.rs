use std::{cell::RefCell, rc::Rc};

use super::*;
use crate::{
    object::{Environment, Object},
    parser,
};

fn test_eval(input: &str) -> Rc<Object> {
    let env = Rc::new(RefCell::new(Environment::new()));
    match parser::parse(input) {
        Ok(node) => eval(&node, env).expect(input),
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

#[test]
fn test_if_else_expression() {
    // Use 0 represent null
    let tests = [
        ("if (true) { 10 }", 10),
        ("if (false) { 10 }", 0),
        ("if (1) { 10 }", 10),
        ("if (1 < 2) { 10 }", 10),
        ("if (1 > 2) { 10 }", 0),
        ("if (1 > 2) { 10 } else { 20 }", 20),
        ("if (1 < 2) { 10 } else { 20 }", 10),
    ];
    for t in tests {
        let evaluated = test_eval(t.0);
        match t.1 {
            0 => test_null_object(&evaluated),
            _ => test_integer_object(&evaluated, t.1),
        }
    }
}

#[test]
fn test_return_statement() {
    let tests = [
        // return
        ("return 10;", 10),
        ("return 10; 9;", 10),
        ("return 2 * 5; 9;", 10),
        ("9; return 2 * 5; 9;", 10),
        (
            r#"if (10 > 1) {
if (10 > 1) {
return 10;
}
return 1;
}"#,
            10,
        ),
        (
            r#"
  if (10 > 1) {
    if (1 > 10) {
      return 10;
    } else {
      return 101;
    }
    return 1;
  }
"#,
            101,
        ),
    ];
    for t in tests {
        let evaluated = test_eval(t.0);
        test_integer_object(&evaluated, t.1)
    }
}

#[test]
fn test_let_statement() {
    let tests = [
        ("let a = 5; a;", 5),
        ("let a = 5 * 5; a;", 25),
        ("let a = 5; let b = a; b;", 5),
        ("let a = 5; let b = a; let c = a + b + 5; c;", 15),
    ];
    for t in tests {
        let evaluated = test_eval(t.0);
        test_integer_object(&evaluated, t.1)
    }
}

#[test]
fn test_error_handling() {
    let tests = [
        ("5 + true;", "type mismatch: Int(5) + Bool(true)"),
        ("5 + true; 5;", "type mismatch: Int(5) + Bool(true)"),
        ("-true", "unknown operator: -Bool(true)"),
        ("true + false", "unknown operator: true + false"),
        ("5; true + false; 5", "unknown operator: true + false"),
        (
            "if (10 > 1) { true + false; }",
            "unknown operator: true + false",
        ),
        (
            "if (10 > 1) {
                             if (10 > 1) {
                                return true + false;
                             }

                             return 1;
                          }",
            "unknown operator: true + false",
        ),
        ("foobar", "identifier not found: foobar"),
        (r#""Hello" - "World""#, "unknown operator: Hello - World"),
        // (
        //     r#" {"name": "Monkey"}[fn(x) { x }]; "#,
        //     "unusable as hash key: fn(x) {\nx\n}",
        // ),
    ];
    for t in tests {
        let env = Rc::new(RefCell::new(Environment::new()));
        match parser::parse(t.0) {
            Ok(node) => match eval(&node, env) {
                Err(e) => assert_eq!(e.message, t.1),
                n => panic!("expected error {} but got {:?}", t.1, n),
            },
            Err(e) => panic!("error {:?} on input {}", e, t.0),
        }
    }
}

#[test]
fn test_function_object() {
    let tests = [(
        "fn(x) { x + 2; };",
        r#"fn(x) {
(x + 2)
}"#,
    )];
    for t in tests {
        let evaluated = test_eval(t.0);
        assert_eq!(evaluated.to_string(), t.1.to_string())
    }
}

#[test]
fn test_function_application() {
    let tests = [
        ("let identity = fn(x) { x; }; identity(5);", 5),
        ("let identity = fn(x) { return x; }; identity(5);", 5),
        ("let double = fn(x) { x * 2; }; double(5);", 10),
        ("let add = fn(x, y) { x + y; }; add(5, 5);", 10),
        ("let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));", 20),
        ("fn(x) { x; }(5)", 5),
    ];

    for t in tests {
        test_integer_object(&test_eval(t.0), t.1)
    }
}

#[test]
fn test_string_literal() {
    let input = r#""Hello World!""#;

    match &*test_eval(input) {
        Object::String(s) => assert_eq!(s, "Hello World!"),
        obj => panic!("expected string but got {:?}", obj),
    }
}

#[test]
fn test_string_concatenation() {
    let input = r#""Hello" + " " + "World!""#;

    match &*test_eval(input) {
        Object::String(s) => assert_eq!(s, "Hello World!"),
        obj => panic!("expected string but got {:?}", obj),
    }
}

#[test]
fn test_builtin_functions() {
    let tests = [
        (r#"len("")"#, "0"),
        (r#"len("four")"#, "4"),
        (r#"len("hello world")"#, "11"),
        // (r#"len(1)"#, "argument to `len` not supported, got INTEGER"),
        // (
        //     r#"len("one", "two")"#,
        //     "len takes only 1 array or string argument",
        // ),
    ];
    for t in tests {
        let obj = test_eval(t.0);
        match *obj {
            Object::Int(i) => assert_eq!(i.to_string(), t.1),
            _ => panic!("expected integer object, but got {:?}", obj),
        }
    }
}

#[test]
fn test_array_literals() {
    let input = "[1, 2 * 2, 3 + 3]";
    let obj = test_eval(input);
    match &*obj {
        Object::Array(a) => {
            test_integer_object(a.elements.get(0).unwrap(), 1);
            test_integer_object(a.elements.get(1).unwrap(), 4);
            test_integer_object(a.elements.get(2).unwrap(), 6);
        }
        _ => panic!("expected array but got {:?}", obj),
    }
}

#[test]
fn test_array_index_expressions() {
    let tests = [
        ("[1, 2, 3][0]", 1),
        ("[1, 2, 3][1]", 2),
        ("[1, 2, 3][2]", 3),
        ("let i = 0; [1][i];", 1),
        ("[1, 2, 3][1 + 1];", 3),
        ("let myArray = [1, 2, 3]; myArray[2];", 3),
        (
            "let myArray = [1, 2, 3]; myArray[0] + myArray[1] + myArray[2];",
            6,
        ),
        ("let myArray = [1, 2, 3]; let i = myArray[0]; myArray[i]", 2),
    ];
    let null_tests = [("[1, 2, 3][3]", "null"), ("[1, 2, 3][-1]", "null")];
    for t in tests {
        test_integer_object(&test_eval(t.0), t.1)
    }
    for t in null_tests {
        assert_eq!(test_eval(t.0).to_string(), t.1);
    }
}

#[test]
fn test_hash_index_expressions() {
    let tests = [
        (r#"{"foo": 4+1}["foo"]"#, 5),
        (r#"{"foo": 6-1}["foo"]"#, 5),
        (r#"let key = "foo"; {"foo": 5}[key]"#, 5),
        (r#"{5: 5}[5*1]"#, 5),
        (r#"{5: 5}[5/1]"#, 5),
        (r#"{true: 5}[true]"#, 5),
        (r#"{false: 5}[false]"#, 5),
    ];
    let null_tests = [(r#"{"foo": 5}["bar"]"#, "null"), (r#"{}["foo"]"#, "null")];
    for t in tests {
        test_integer_object(&test_eval(t.0), t.1)
    }
    for t in null_tests {
        assert_eq!(test_eval(t.0).to_string(), t.1);
    }
}
