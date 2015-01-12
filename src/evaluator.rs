use parser::Sexpr;
use std::fmt;

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
      LispValue::Nil => "nil".to_string(),
      LispValue::Int(x) => x.to_string(),
      LispValue::Symbol(ref v) => format!("{}", v),
    }
  }
}

impl fmt::String for LispValue {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    write!(fmt, "{}", self.pretty_print())
  }
}


impl Evaluator {
  pub fn new() -> Evaluator {
    Evaluator
  }

  pub fn eval(&self, sexpr: Sexpr) -> EvalResult {
    let val = match sexpr {
      Sexpr::Int(x) => Ok(LispValue::Int(x)),
      Sexpr::Symbol(x) => Ok(LispValue::Symbol(x)),
      Sexpr::List(_) => self.eval_list(sexpr),
      _ => Ok(LispValue::Nil)
    };
    //println!("eval value - {}", val);
    val
  }

  fn eval_list(&self, list: Sexpr) -> EvalResult {
    match list {
      Sexpr::List(ref children) if children.len() == 0 => Ok(LispValue::Nil),
      Sexpr::List(ref children) if children.len() == 1 => self.eval(*children[0].clone()),
      Sexpr::List(children) => {
        let operator = try!(self.eval(*children[0].clone()));
        // TODO: Ensure operator is Symbol
        let mut x = try!(self.eval(*children[1].clone()));
        for child in children.slice_from(2).iter() {
          x = try!(self.eval_op(&operator, x, try!(self.eval(*child.clone()))));
        }
        Ok(x)
      },
      _ => Ok(LispValue::Nil) // FIXME
    }
  }

  fn eval_op(&self, operator: &LispValue, x: LispValue, y: LispValue) -> Result<LispValue, String> {
    let op = match *operator {
      LispValue::Symbol(ref o) => o,
      _ => return Err("Wrong first operator".to_string())
    };

    let y_unwrap = match y {
      LispValue::Int(i) => i,
      _ => return Err("Support only ints now".to_string())
    };

    let x_unwrap = match x {
      LispValue::Int(i) => i,
      _ => return Err("Support only ints now".to_string())
    };

    match op.as_slice() {
     "+" => Ok(LispValue::Int(x_unwrap + y_unwrap)),
     "-" => Ok(LispValue::Int(x_unwrap - y_unwrap)),
     "*" => Ok(LispValue::Int(x_unwrap * y_unwrap)),
     "/" => match y_unwrap {
        y_unwrap if y_unwrap == 0 => Err("Division by zero".to_string()),
        _ => Ok(LispValue::Int(x_unwrap / y_unwrap))
     },
      _ => Err("Not supported yet".to_string())
    }
  }
}
