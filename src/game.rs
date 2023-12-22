use std::io::{self, Write};

use rand::seq::SliceRandom;
use rand::Rng;

use crate::grid;

pub enum Difficulty {
  Easy,
  Medium,
  Hard,
}

pub struct Game {
  pub grid: grid::Grid,
  pub difficulty: Difficulty,
}

impl Game {
  pub fn new(difficulty: String) -> Game {
    Game {
      grid: grid::Grid::new(9, 9),
      difficulty: match difficulty.as_str() {
        "easy" => Difficulty::Easy,
        "medium" => Difficulty::Medium,
        "hard" => Difficulty::Hard,
        _ => Difficulty::Easy,
      },
    }
  }

  pub fn init(&mut self) {
    self.generate_full_grid();
    self.remove_cells();
  }

  pub fn generate_full_grid(&mut self) -> bool {
    let mut rng = rand::thread_rng();
    let mut values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    values.shuffle(&mut rng);

    for x in 0..9 {
      for y in 0..9 {
        if self.grid.get_cell(x, y).value.is_none() {
          for val in values.iter() {
            if self.grid.is_valid(x, y, *val) {
              self.grid.set_default(x, y, *val);

              if self.generate_full_grid() {
                return true;
              }

              self.grid.set_empty(x, y);
            }
          }

          return false;
        }
      }
    }

    true
  }

  pub fn remove_cells(&mut self) {
    let mut rng = rand::thread_rng();

    let mut count = match self.difficulty {
      Difficulty::Easy => rng.gen_range(40..50),
      Difficulty::Medium => rng.gen_range(50..60),
      Difficulty::Hard => rng.gen_range(60..70),
    };

    while count > 0 {
      let x = rng.gen_range(0..9);
      let y = rng.gen_range(0..9);

      if self.grid.is_empty(x, y) {
        continue;
      }

      self.grid.set_empty(x, y);
      count -= 1;
    }
  }

  pub fn display_grid(&self) {
    print!("{}", self.grid.to_string());
  }

  pub fn is_solved(&mut self) -> bool {
    self.grid.is_solved()
  }

  pub fn prompt(&mut self) {
    print!("> ");
    let _ = io::stdout().flush();

    let mut input = String::new();

    io::stdin()
      .read_line(&mut input)
      .expect("Failed to read line");

    let input = input.trim();

    let mut words = input.split_whitespace();

    match words.next() {
      Some("help") => self.prompt_help(),
      Some("set") => {
        let x = match words.next() {
          Some(x) => match x.parse::<u32>() {
            Ok(x) => x,
            Err(_) => {
              println!("Invalid x coordinate");
              return;
            }
          },
          None => {
            println!("Missing x coordinate");
            return;
          }
        };

        let y = match words.next() {
          Some(y) => match y.parse::<u32>() {
            Ok(y) => y,
            Err(_) => {
              println!("Invalid y coordinate");
              return;
            }
          },
          None => {
            println!("Missing y coordinate");
            return;
          }
        };

        let value = match words.next() {
          Some(value) => match value.parse::<u32>() {
            Ok(value) => value,
            Err(_) => {
              println!("Invalid value");
              return;
            }
          },
          None => {
            println!("Missing value");
            return;
          }
        };

        if x < 1 || x > 9 {
          println!("Invalid x coordinate");
          return;
        }

        if y < 1 || y > 9 {
          println!("Invalid y coordinate");
          return;
        }

        if value < 1 || value > 9 {
          println!("Invalid value");
          return;
        }

        if !self.grid.can_set(x - 1, y - 1) {
          println!("You can't change this cell");
          return;
        }

        if !self.grid.is_empty(x - 1, y - 1) {
          println!("This cell is already set");
          return;
        }

        self.grid.set_value(x - 1, y - 1, value);
      }
      Some("solve") => {
        if self.is_solved() {
          println!("You already solved the grid");
          return;
        }
      }
      Some("quit") => {
        println!("Bye!");
        std::process::exit(0);
      }
      _ => {
        println!("Unknown command");
      }
    }
  }

  pub fn prompt_help(&mut self) {
    println!("Enter a command:");
    println!("  - help");
    println!("  - set x y value");
    println!("  - solve");
    println!("  - quit");
  }

  pub fn run(&mut self) {
    self.prompt_help();

    while self.grid.count_empty() != 0 {
      self.display_grid();
      self.prompt();
    }


    if self.is_solved() {
      println!("You solved the grid!");
    } else {
      println!("You didn't solve the grid!");
    }
  }
}