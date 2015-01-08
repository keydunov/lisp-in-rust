use std::io::stdio;
use evaluator::Evaluator;

mod parser;
mod evaluator;

// Try to use later https://github.com/shaleh/rust-readline
fn process_line(line: String, evaluator: &Evaluator) {
  match parser::parse(line) {
    Ok(sexpr) => {
      match evaluator.eval(sexpr) {
        Ok(val) => println!("==> {}", val),
        Err(e) => println!("error: {}", e)
      }
    }
    Err(e) => println!("parse error: {}", e)
  }
}

fn repl() {
  let evaluator = Evaluator::new();
  loop {
    print!("lisp> ");
    let input = stdio::stdin().read_line();

    match input {
      Ok(line) => process_line(line, &evaluator),
      Err(_) => {
        println!("Quiting...");
        break;
      }
    }
  }
}

fn main() {
  println!("Lisp repl in Rust by Artyom Keydunov");
  repl();
}
