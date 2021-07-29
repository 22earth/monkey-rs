use super::*;

fn setup(input: &str, stmt_count: usize) -> Program {
    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let prog = p.parse_program().unwrap();

    if stmt_count != 0 && prog.body.len() != stmt_count {
        panic!(
            "expected 1 statement for '{}' but got {:?}",
            input, prog.body
        )
    }

    prog
}

#[test]
fn test_return_statement() {
    let input = r#"
return bar;
return 5;
return true;
return "foo";
"#;
    let prog = setup(input, 4);
    let tests = [
        Expression::Identifier("bar".to_string()),
        Expression::Integer(5),
        Expression::Boolean(true),
        Expression::String("foo".to_string()),
    ];
    let mut it = prog.body.iter();
    for t in tests {
        match it.next().unwrap() {
            node::Statement::Return(ref l) => {
                assert_eq!(l.value, t);
            }
            _ => panic!("invalid node"),
        }
    }
}

#[test]
fn test_let_statement() {
    let input = r#"let foo = bar;
let foo = 5;
let foo = true;
let foo = "foo";
"#;
    let prog = setup(input, 4);
    let tests = [
        Expression::Identifier("bar".to_string()),
        Expression::Integer(5),
        Expression::Boolean(true),
        Expression::String("foo".to_string()),
    ];
    let mut it = prog.body.iter();
    for t in tests {
        match it.next().unwrap() {
            node::Statement::Let(ref l) => {
                assert_eq!(l.name, "foo".to_string());
                assert_eq!(l.value, t);
            }
            _ => panic!("invalid node"),
        }
    }
}

#[test]
fn test_prefix_expression() {
    let input = r#"!5;
-15;
"#;
    let prog = setup(input, 2);
    let tests = [
        Expression::Prefix(Box::new(node::PrefixExpression {
            operator: TokenKind::punctuator(Punctuator::Not),
            right: Expression::Integer(5),
        })),
        Expression::Prefix(Box::new(node::PrefixExpression {
            operator: TokenKind::punctuator(Punctuator::Sub),
            right: Expression::Integer(15),
        })),
    ];
    let mut it = prog.body.iter();
    for t in tests {
        match it.next().unwrap() {
            node::Statement::Expression(ref l) => {
                assert_eq!(l.expression, t);
            }
            _ => panic!("invalid node"),
        }
    }
}

#[test]
fn test_infix_expression() {
    let input = r#"5 + 5;
5 - 5;
5 * 5;
5 / 5;
5 > 5;
5 < 5;
5 == 5;
5 != 5;
"#;
    let prog = setup(input, 8);
    let tests = [
        Expression::Infix(Box::new(node::InfixExpression {
            left: Expression::Integer(5),
            right: Expression::Integer(5),
            operator: TokenKind::punctuator(Punctuator::Add),
        })),
        Expression::Infix(Box::new(node::InfixExpression {
            left: Expression::Integer(5),
            right: Expression::Integer(5),
            operator: TokenKind::punctuator(Punctuator::Sub),
        })),
        Expression::Infix(Box::new(node::InfixExpression {
            left: Expression::Integer(5),
            right: Expression::Integer(5),
            operator: TokenKind::punctuator(Punctuator::Mul),
        })),
        Expression::Infix(Box::new(node::InfixExpression {
            left: Expression::Integer(5),
            right: Expression::Integer(5),
            operator: TokenKind::punctuator(Punctuator::Div),
        })),
        Expression::Infix(Box::new(node::InfixExpression {
            left: Expression::Integer(5),
            right: Expression::Integer(5),
            operator: TokenKind::punctuator(Punctuator::GreaterThan),
        })),
        Expression::Infix(Box::new(node::InfixExpression {
            left: Expression::Integer(5),
            right: Expression::Integer(5),
            operator: TokenKind::punctuator(Punctuator::LessThan),
        })),
        Expression::Infix(Box::new(node::InfixExpression {
            left: Expression::Integer(5),
            right: Expression::Integer(5),
            operator: TokenKind::punctuator(Punctuator::Eq),
        })),
        Expression::Infix(Box::new(node::InfixExpression {
            left: Expression::Integer(5),
            right: Expression::Integer(5),
            operator: TokenKind::punctuator(Punctuator::NotEq),
        })),
    ];
    let mut it = prog.body.iter();
    for t in tests {
        match it.next().unwrap() {
            node::Statement::Expression(ref l) => {
                assert_eq!(l.expression, t);
            }
            _ => panic!("invalid node"),
        }
    }
}

#[test]
fn test_operator_precedence() {
    let tests = vec![
        ("-a * b", "((-a) * b)"),
        ("!-a", "(!(-a))"),
        ("a + b + c", "((a + b) + c)"),
        ("a + b - c", "((a + b) - c)"),
        ("a * b * c", "((a * b) * c)"),
        ("a * b / c", "((a * b) / c)"),
        ("a + b / c", "(a + (b / c))"),
        ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
        ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
        ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
        ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
        (
            "3 + 4 * 5 == 3 * 1 + 4 * 5",
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
        ),
        (
            "3 + 4 * 5 == 3 * 1 + 4 * 5",
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
        ),
        // test grouped expression
        ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
        ("(5 + 5) * 2", "((5 + 5) * 2)"),
        ("2 / (5 + 5)", "(2 / (5 + 5))"),
        ("-(5 + 5)", "(-(5 + 5))"),
        ("!(true == true)", "(!(true == true))"),
    ];
    for t in tests {
        let l = Lexer::new(t.0);
        let mut p = Parser::new(l);
        let prog = p.parse_program().unwrap();
        assert_eq!(prog.to_string(), t.1);
    }
}

#[test]
fn test_if_expression() {
    let tests = vec![
        ("if (x < y) { x }", "if (x < y) x"),
        ("if (x < y) { x } else { y }", "if (x < y) x else y"),
    ];
    for t in tests {
        let l = Lexer::new(t.0);
        let mut p = Parser::new(l);
        let prog = p.parse_program().unwrap();
        assert_eq!(prog.to_string(), t.1);
    }
}

#[test]
fn test_fn_literal() {
    let tests = vec![("fn(x, y) { x + y; }", "(x, y) (x + y)")];
    for t in tests {
        let l = Lexer::new(t.0);
        let mut p = Parser::new(l);
        let prog = p.parse_program().unwrap();
        assert_eq!(prog.to_string(), t.1);
    }
}
