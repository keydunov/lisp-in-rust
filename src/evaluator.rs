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
    let val = match sexpr {
      parser::Int(x) => Ok(Int(x)),
      parser::Symbol(x) => Ok(Symbol(x)),
      parser::List(_) => self.eval_list(sexpr),
      _ => Ok(Nil)
    };
    //println!("eval value - {}", val);
    val
  }

  fn eval_list(&self, list: parser::Sexpr) -> EvalResult {
    match list {
      parser::List(ref children) if children.len() == 0 => Ok(Nil),
      parser::List(ref children) if children.len() == 1 => self.eval(*children[0].clone()),
      parser::List(children) => {
        let operator = try!(self.eval(*children[0].clone()));
        // TODO: Ensure operator is Symbol
        let mut x = try!(self.eval(*children[1].clone()));
        for child in children.slice_from(2).iter() {
          x = try!(self.eval_op(&operator, x, try!(self.eval(*child.clone()))));
        }
        Ok(x)
      },
      _ => Ok(Nil) // FIXME
    }
  }

  fn eval_op(&self, operator: &LispValue, x: LispValue, y: LispValue) -> Result<LispValue, String> {
    let op = match *operator {
      Symbol(ref o) => o,
      _ => return Err("Wrong first operator".to_string())
    };

    let y_unwrap = match y {
      Int(i) => i,
      _ => return Err("Support only ints now".to_string())
    };

    let x_unwrap = match x {
      Int(i) => i,
      _ => return Err("Support only ints now".to_string())
    };

    match op.as_slice() {
     "+" => Ok(Int(x_unwrap + y_unwrap)),
     "-" => Ok(Int(x_unwrap - y_unwrap)),
     "*" => Ok(Int(x_unwrap * y_unwrap)),
     "/" => match y_unwrap {
        y_unwrap if y_unwrap == 0 => Err("Division by zero".to_string()),
        _ => Ok(Int(x_unwrap / y_unwrap))
     },
      _ => Err("Not supported yet".to_string())
    }
  }
}
