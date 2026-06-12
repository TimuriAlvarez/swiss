use gprl::types::Res;
use std::io::Write;
use crate::runner::{spawn, Program::Glow, Program::More};
use tracing::{event, Level};

fn confirm_book_execution() -> Res::<bool> {
  print!(":: Proceed with running the book? [Y/n] ");
  std::io::stdout().flush()?;
  let mut input: String = String::new();
  std::io::stdin().read_line(&mut input)?;
  input = input.trim().to_lowercase();
  Ok(input == "" || input == "y")
}

fn display_book_script(text: &String) {
  if crate::runner::run(spawn, Glow, &["--tui"], Some(format!("```justfile\n{text}\n```\n")), &[]).is_ok() { return }
  event!(Level::WARN, "Missing '{Glow}' executable, falling back to '{More}'");
  if crate::runner::run(spawn, More, &["--silent", "--clean-print"], Some(text.to_string()), &[]).is_ok() { return }
  event!(Level::WARN, "Missing '{More}' executable, falling back to 'stdout'");
  println!("{text}");
}

pub fn confirm(_book: &String, text: &String) -> Res::<bool> {
  display_book_script(text);
  confirm_book_execution()
}
