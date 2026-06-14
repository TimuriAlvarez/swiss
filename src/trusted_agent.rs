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

fn display_book_script(temp_file: &temp_file::TempFile) {
  if crate::runner::runner(spawn, Glow, &["--tui", "--width", "0"], Some(&temp_file), &[]).is_ok() { return }
  if crate::runner::runner(spawn, More, &["--silent", "--clean-print"], Some(&temp_file), &[]).is_ok() { return }
  let text: String = std::fs::read_to_string(temp_file).unwrap_or_default();
  println!("{text}");
}

pub fn confirm(_book: &String, temp_file: &temp_file::TempFile) -> Res::<bool> {
  display_book_script(temp_file);
  confirm_book_execution()
}
