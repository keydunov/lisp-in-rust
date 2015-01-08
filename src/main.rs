extern crate readline;

use evaluator::Evaluator;

mod parser;
mod evaluator;

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
    let input = readline::readline("lisp> ");

    match input {
      Ok(line) => {
        readline::add_history(line.as_slice());
        process_line(line, &evaluator);
      }
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
