use std::io;

use monkey_rs::repl;

fn main() -> io::Result<()> {
    // TODO get user name
    println!("This is the Monkey programming language!");
    let input = io::stdin();
    let output = io::stdout();

    repl::start(input.lock(), output.lock())
}
