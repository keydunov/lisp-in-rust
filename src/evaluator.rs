use parser;
use std::fmt::{Formatter, FormatError, Show};

pub enum LispValue {
  Nil,
  Symbol(String),
  Int(i32)
}

pub struct Evaluator;

pub type EvalResult = Result<LispValue, String>;

impl LispValue {
  fn pretty_print(&self) -> String {
    match *self {
      Nil => "nil".to_string(),
      Int(x) => x.to_string(),
      Symbol(ref v) => format!("{}", v),
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
      parser::Int(x) => Ok(Int(x)),
      parser::Symbol(x) => Ok(Symbol(x)),
      parser::List(children) => {
        let operator = try!(self.eval(*children[0].clone()));
        let mut result = 0;
        for child in children.tail().iter() {
          result = try!(self.eval_op(&operator, result, try!(self.eval(*child.clone()))));
        }
        Ok(Int(result))
      }
      _ => Ok(Nil)
    }
  }

  fn eval_op(&self, operator: &LispValue, result: i32, y: LispValue) -> Result<i32, String> {
    let op = match *operator {
      Symbol(ref o) => o,
      _ => return Err("Wrong first operator".to_string())
    };

    let y_unwrap = match y {
      Int(i) => i,
      _ => return Err("Support only ints now".to_string())
    };

    match op.as_slice() {
     "+" => Ok(result + y_unwrap),
     "-" => Ok(result - y_unwrap),
     "*" => Ok(result * y_unwrap),
     "/" => Ok(result / y_unwrap),
      _ => Err("Not supported yet".to_string())
    }
  }
}
