use std::fmt::Display;

use crate::cell::Cell;

pub struct Grid {
  pub width: u32,
  pub height: u32,
  pub cells: Vec<Cell>,
}

impl Grid {
  pub fn new(width: u32, height: u32) -> Grid {
    let mut cells = Vec::new();
    for _ in 0..height {
      for _ in 0..width {
        cells.push(Cell::new_empty());
      }
    }
    Grid {
      width,
      height,
      cells,
    }
  }

  pub fn is_valid(&mut self, x: u32, y: u32, value: u32) -> bool {
    for i in 0..self.width {
      if self.get_cell(i, y).get_value() == value || self.get_cell(x, i).get_value() == value {
        return false;
      }
    }

    let start_x = (x / 3) * 3;
    let start_y = (y / 3) * 3;

    for x in 0..3 {
      for y in 0..3 {
        if self.get_cell(start_x + x, start_y + y).get_value() == value {
          return false;
        }
      }
    }

    true
  }

  pub fn get_cell(&self, x: u32, y: u32) -> &Cell {
    &self.cells[(y * self.width + x) as usize]
  }

  pub fn get_cell_mut(&mut self, x: u32, y: u32) -> &mut Cell {
    &mut self.cells[(y * self.width + x) as usize]
  }

  pub fn set_value(&mut self, x: u32, y: u32, value: u32) {
    self.get_cell_mut(x, y).set_value(value);
  }

  pub fn set_default(&mut self, x: u32, y: u32, value: u32) {
    self.get_cell_mut(x, y).set_value(value);
    self.get_cell_mut(x, y).can_set = false;
  }

  pub fn set_empty(&mut self, x: u32, y: u32) {
    self.get_cell_mut(x, y).set_empty();
  }

  pub fn is_empty(&self, x: u32, y: u32) -> bool {
    self.get_cell(x, y).value.is_none()
  }

  pub fn can_set(&self, x: u32, y: u32) -> bool {
    self.get_cell(x, y).can_set
  }

  fn check_row(&mut self, y: u32) -> bool {
    let mut row = Vec::new();
    for x in 0..self.width {
      row.push(self.get_cell(x, y).get_value());
    }

    let sum = row.iter().sum::<u32>();

    return sum == 45
  }

  fn check_column(&mut self, x: u32) -> bool {
    let mut column = Vec::new();
    for y in 0..self.height {
      column.push(self.get_cell(x, y).get_value());
    }

    let sum = column.iter().sum::<u32>();

    return sum == 45
  }

  fn check_square(&mut self, x: u32, y: u32) -> bool {
    let mut square = Vec::new();
    for y in y..y + 3 {
      for x in x..x + 3 {
        square.push(self.get_cell(x, y).get_value());
      }
    }

    let sum = square.iter().sum::<u32>();

    return sum == 45
  }

  pub fn is_solved(&mut self) -> bool {
    for y in 0..self.height {
      if !self.check_row(y) {
        return false;
      }
    }

    for x in 0..self.width {
      if !self.check_column(x) {
        return false;
      }
    }

    for y in (0..self.height).step_by(3) {
      for x in (0..self.width).step_by(3) {
        if !self.check_square(x, y) {
          return false;
        }
      }
    }

    return true;
  }

  pub fn count_empty(&self) -> u32 {
    let mut count = 0;
    for y in 0..self.height {
      for x in 0..self.width {
        if self.is_empty(x, y) {
          count += 1;
        }
      }
    }
    count
  }
}

impl Display for Grid {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut output = String::new();
    output.push_str("   1 2 3   4 5 6   7 8 9\n");
    output.push_str(" +-------+-------+-------+\n");

    for y in 0..self.height {
      output.push_str(format!("{}| ", y + 1).as_str());

      for x in 0..self.width {

        let cell = self.get_cell(x, y);
        output.push_str(&format!("{} ", cell));

        if x == 2 || x == 5 {
          output.push_str("| ");
        }
      }

      output.push_str("|\n");

      if y == 2 || y == 5 {
        output.push_str(" +-------+-------+-------+\n");
      }
    }

    output.push_str(" +-------+-------+-------+\n");

    write!(f, "{}", output)
  }
}