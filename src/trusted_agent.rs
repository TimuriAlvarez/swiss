use gprl::types::Res;
use std::io::Write;
use crate::runner::{spawn, Program::Glow, Program::More};

fn confirm_book_execution() -> Res::<bool> {
  print!(":: Do you want to run this recipe book? [Y/n] ");
  std::io::stdout().flush()?;
  let mut input: String = String::new();
  std::io::stdin().read_line(&mut input)?;
  input = input.trim().to_lowercase();
  Ok(input == "" || input == "y")
}

fn display_book_script(text: &String) {
  if crate::runner::runner(spawn, Glow, &["--tui", "--width", "0"], Some(&format!("```justfile\n{text}\n```\n")), &[]).is_ok() { return }
  if crate::runner::runner(spawn, More, &["--silent", "--clean-print"], Some(text), &[]).is_ok() { return }
  println!("{text}");
}

pub fn confirm(_book: &String, text: &String) -> Res::<bool> {
  display_book_script(text);
  confirm_book_execution()
}
