#[derive(Show, Clone)]
pub enum Sexpr {
  Nil,
  Int(i32),
  Symbol(String),
  Cons(Box<Sexpr>, Box<Sexpr>),
  List(Vec<Box<Sexpr>>),
}

type ParseResult = Result<Sexpr, String>;

struct Parser {
  pos: usize,
  input: String
}

/// Parse a string and return ParseResult - either Sexpr AST or Error.
pub fn parse(source: String) -> ParseResult {
    let mut parser = Parser { pos: 0us, input: source };
    parser.parse()
}

trait Identifiable {
  fn is_valid_for_identifier(&self) -> bool;
}

impl Identifiable for char {
  fn is_valid_for_identifier(&self) -> bool {
      match *self {
          '!' | '$' | '%' | '&' | '*' | '+' | '-' | '.' | '~' |
          '/' | ':' | '<' | '=' | '>' | '?' | '@' | '^' | '_' |
          'a'...'z' | 'A'...'Z' | '0'...'9' => true,
          _ => false
      }
  }
}

impl Parser {
  fn parse(&mut self) -> ParseResult {
    self.consume_whitespace();
    if self.eof() {
      Ok(Sexpr::Nil)
    } else {
      self.do_parse()
    }
  }

  fn do_parse(&mut self) -> ParseResult {
    match self.next_char() {
      c if c.is_digit(10) => self.parse_number(),
      '(' => {
        self.consume_char();
        self.parse_list()
      }
      _ => self.parse_symbol()
    }
  }

  fn parse_symbol(&mut self) -> ParseResult {
    let symbol = self.consume_while(|char| { char.is_valid_for_identifier() });
    self.consume_whitespace();

    match symbol.as_slice() {
      "nil" => Ok(Sexpr::Nil),
      _     => Ok(Sexpr::Symbol(symbol))
    }
  }

  fn parse_number(&mut self) -> ParseResult {
    let string = self.consume_while(|char| { char.is_digit(10) });
    self.consume_whitespace();
    Ok(Sexpr::Int(string.parse::<i32>().unwrap()))
  }

  fn parse_list(&mut self) -> ParseResult {
    let mut children = vec!();
    self.consume_whitespace();
    while self.next_char() != ')' {
      children.push(Box::new(try!(self.parse())))
    }

    Ok(Sexpr::List(children))
  }

  /// Consume and discard zero or more whitespace characters.
  fn consume_whitespace(&mut self) {
      self.consume_while(|c| c.is_whitespace());
  }

  /// Consume characters until `test` returns false.
  fn consume_while<F>(&mut self, test: F) -> String where F : Fn(char) -> bool {
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

  #[test]
  fn parses_symbols() {
      let result = parse("hello".to_string());
      assert!(result.is_ok());
      let sexp = result.unwrap();
      match sexp {
          Symbol(x) => assert_eq!(x, "hello".to_string()),
          _ => fail!("Parsed incorrectly, got {}", sexp)
      };
  }

  #[test]
  fn parses_lists() {
      let result = parse("(1 2 3)".to_string());
      assert!(result.is_ok(), "parse failed: {}", result);
      let sexp = result.unwrap();
      match sexp {
          List(children) => {
            match children[2] {
              box Int(x) => assert_eq!(x, 3),
              _ => fail!("Parsed incorrectly, got {}", children[0])
              }
          },
          _ => fail!("Parsed incorrectly, got {}", sexp)
      };
  }
}
