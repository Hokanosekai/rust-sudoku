mod grid;
mod cell;
mod game;
mod cli;

use game::Game;
use cli::Cli;

use clap::Parser;

fn main() {

  let cli = Cli::parse();
  let difficulty = cli.difficulty.unwrap();

  let mut game = Game::new(difficulty);
  game.init();

  game.run();
}

