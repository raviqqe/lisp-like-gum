use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

use self::LoadError::*;



#[derive(Debug)]
pub enum LoadError {
  TypeMismatch,
  NotCached,
}

impl Display for LoadError {
  fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
    write!(f, "{:?}", *self)
  }
}

impl Error for LoadError {
  fn description(&self) -> &str {
    match *self {
      TypeMismatch => "The type of an object you tried to load and the \
                       specified one don't match.",
      NotCached => "The object you tried to load is not cached yet.",
    }
  }
}
