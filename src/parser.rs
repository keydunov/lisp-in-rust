pub enum Sexpr {
  Nil,
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
    Ok(Nil)
  }
}
