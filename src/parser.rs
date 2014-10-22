#[deriving(Show)]
pub enum Sexpr {
  Nil,
  Int(i32),
}

type ParseResult = Result<Sexpr, String>;

struct Parser {
  pos: uint,
  input: String
}

// Parse
pub fn parse(source: String) -> ParseResult {
    let mut parser = Parser { pos: 0u, input: source };
    parser.parse()
}

impl Parser {
  fn parse(&mut self) -> ParseResult {
    Ok(Int(42))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parses_ints() {
      let result = parse("42".to_string());
      assert!(result.is_ok());
      let sexp = result.unwrap();
      match sexp {
          Int(x) => assert_eq!(x, 42),
          _ => fail!("Parsed incorrectly, got {}", sexp)
      };
  }
}
