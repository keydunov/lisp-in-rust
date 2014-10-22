use parser;
use std::fmt::{Formatter, FormatError, Show};

pub enum LispValue {
  Nil
}

pub struct Evaluator;

pub type EvalResult = Result<LispValue, String>;

impl LispValue {
  fn pretty_print(&self) -> &str {
    match *self {
      Nil => "nil"
    }
  }
}

impl Show for LispValue {
  fn fmt(&self, fmt: &mut Formatter) -> Result<(), FormatError> {
    write!(fmt, "{}", self.pretty_print())
  }
}


impl Evaluator {
  pub fn new() -> Evaluator {
    Evaluator
  }

  pub fn eval(&self, sexpr: parser::Sexpr) -> EvalResult {
    match sexpr {
      parser::Nil => Ok(Nil)
    }
  }
}
