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
fn test_let_statement() {
    let input = r"
let x = 5;
let y = 10;
let foobar = 838383;";
    let prog = setup(input, 3);
    let tests = ["x", "y", "foobar"];
    let mut it = prog.body.iter();
    for t in tests {
        match it.next().unwrap() {
            node::Statement::Let(ref l) => {
                assert_eq!(l.name, t);
            }
            _ => panic!("invalid node"),
        }
    }
}
