use std::{cell::RefCell, io, rc::Rc};

use crate::{evaluator, object::Environment, parser};

pub fn start<R: io::BufRead, W: io::Write>(mut reader: R, mut writer: W) -> io::Result<()> {
    #![allow(warnings)]
    let env = Rc::new(RefCell::new(Environment::new()));
    loop {
        writer.write(b"> ");
        writer.flush();
        let mut line = String::new();
        reader.read_line(&mut line)?;

        if line.trim_end().is_empty() {
            println!("bye");
            std::process::exit(0)
        }
        match parser::parse(&line) {
            Ok(node) => {
                let evaluated = evaluator::eval(&node, Rc::clone(&env));
                match evaluated {
                    Ok(obj) => {
                        write!(writer, "{}\n", obj);
                    }
                    Err(err) => {
                        write!(writer, "parse errors:\n{}\n", err.to_string());
                    }
                }
            }
            Err(errors) => {
                for err in errors {
                    write!(writer, "parse errors:\n{}\n", err.to_string());
                }
            }
        }
    }
    Ok(())
}
