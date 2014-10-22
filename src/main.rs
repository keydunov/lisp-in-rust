use std::io::stdio;

fn repl() {
  loop {
    print!("lisp> ");
    let input = stdio::stdin().read_line();

    match input {
      Ok(line) => println!("==> {}", line.as_slice().trim()),
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
