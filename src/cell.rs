use std::fmt::{Formatter, Display};

pub struct Cell {
  pub value: Option<u32>,
  pub can_set: bool,
}

impl Cell {
  pub fn new(value: u32) -> Cell {
    Cell {
      value: Some(value),
      can_set: false,
    }
  }

  pub fn new_empty() -> Cell {
    Cell {
      value: None,
      can_set: true,
    }
  }

  pub fn set_empty(&mut self) {
    self.value = None;
    self.can_set = true;
  }

  pub fn set_value(&mut self, value: u32) {
    self.value = Some(value);
  }

  pub fn get_value(&self) -> u32 {
    self.value.or(Some(0)).unwrap()
  }
}

impl Display for Cell {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self.value {
      Some(value) => {
        if self.can_set {
          write!(f, "\x1b[1;34m{}\x1b[0m", value)
        } else {
          write!(f, "{}", value)
        }
      }
      None => write!(f, " "),
    }
  }
}