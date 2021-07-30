use std::io::stdin;

use monkey_rs::{
    lexer::{token::TokenKind, Lexer},
    parser::Parser,
};

fn main() {
    // TODO get user name
    println!("This is the Monkey programming language!");
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        if input.trim_end().is_empty() {
            println!("bye");
            std::process::exit(0)
        }

        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap();
        println!("{}", program);
    }
}
