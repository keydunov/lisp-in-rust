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

/// Parse a string and return ParseResult - either Sexpr AST or Error.
pub fn parse(source: String) -> ParseResult {
    let mut parser = Parser { pos: 0u, input: source };
    parser.parse()
}

impl Parser {
  fn parse(&mut self) -> ParseResult {
    match self.next_char() {
      c if c.is_digit() => self.parse_number(),
      _ => Ok(Nil)

    }
  }

  fn parse_number(&mut self) -> ParseResult {
    let string = self.consume_while(|char| { char.is_digit() });
    Ok(Int(from_str::<i32>(string.as_slice()).unwrap()))
  }

  /// Consume and discard zero or more whitespace characters.
  fn consume_whitespace(&mut self) {
      self.consume_while(|c| c.is_whitespace());
  }

  /// Consume characters until `test` returns false.
  fn consume_while(&mut self, test: |char| -> bool) -> String {
      let mut result = String::new();
      while !self.eof() && test(self.next_char()) {
          result.push(self.consume_char());
      }
      return result;
  }

  /// Return the current character, and advance self.pos to the next character.
  fn consume_char(&mut self) -> char {
      let range = self.input.as_slice().char_range_at(self.pos);
      self.pos = range.next;
      return range.ch;
  }

  /// Read the current character without consuming it.
  fn next_char(&self) -> char {
      self.input.as_slice().char_at(self.pos)
  }

  /// Does the current input start with the given string?
  fn starts_with(&self, s: &str) -> bool {
      self.input.as_slice().slice_from(self.pos).starts_with(s)
  }

  /// Return true if all input is consumed.
  fn eof(&self) -> bool {
      self.pos >= self.input.len()
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
