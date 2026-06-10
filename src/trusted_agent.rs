use gprl::types::Res;
use std::io::Write;
use crate::runner::{Program::Glow, spawn};
use tracing::{event, Level};

fn confirm_book_execution() -> Res::<bool> {
  print!(":: Proceed with running the book? [Y/n] ");
  std::io::stdout().flush()?;
  let mut input: String = String::new();
  std::io::stdin().read_line(&mut input)?;
  input = input.trim().to_lowercase();
  Ok(input == "" || input == "y")
}

pub fn confirm(_book: &String, text: &String) -> Res::<bool> {
  if crate::runner::check(Glow) {
    crate::runner::run(Glow, &["--tui"], Some(format!("```justfile\n{text}\n```\n")), &[], spawn)?;
  } else {
    event!(Level::ERROR, ":: {Glow} app is not installed");
  }
  confirm_book_execution()
}
