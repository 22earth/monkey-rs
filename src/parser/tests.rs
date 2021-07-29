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
    let input = r"
return 5;
return 10;
return 993322;
";
    let prog = setup(input, 3);
    let vals: [i64; 3] = [5, 10, 993322];
    let mut it = prog.body.iter();
    for t in vals {
        match it.next().unwrap() {
            node::Statement::Return(ref l) => {
                assert_eq!(l.value, t.into());
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
