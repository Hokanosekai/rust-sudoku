use clap::Parser;

#[derive(Parser)]
#[command(
  version = "0.1.0",
  author = "Hokanosekai",
  about = "Sudoku game",
  long_about = "Sudoku game implemented in Rust."
)]
pub struct Cli {
  #[arg(short, long, default_value = "easy")]
  pub difficulty: Option<String>,
}