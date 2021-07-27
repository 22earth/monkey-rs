use std::io::stdin;

use monkey_rs::lexer::{token::TokenKind, Lexer};

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

        let mut l = Lexer::new(&input);
        loop {
            let t = l.next_token();
            if t.kind == TokenKind::EOF {
                break;
            } else {
                println!("{}", t)
            }
        }
    }
}
