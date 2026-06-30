use gprl::types::Res;
use std::io::Write;
use crate::runner::{spawn, Program::Glow, Program::More};
use tracing::{event, Level};

fn confirm_book_prompt(prompt: &str) -> Res::<bool> {
  print!(":: {prompt}? [Y/n] ");
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

fn hash(book: &String, text: &String) -> Res<(String, String)> {
  let book: String = blake3::hash(&book.clone().into_bytes()).to_string();
  let hash: String = blake3::hash(&text.clone().into_bytes()).to_string();
  Ok((book, hash))
}

pub const PROMPT: &str = "Do you want to mark this recipe book as trusted";

pub fn confirm(book: &String, text: &String, temp_file: &temp_file::TempFile, prompt: &str) -> Res::<bool> {
  let book_hash: (String, String) = hash(book, text)?;
  let path: std::path::PathBuf = crate::xdg::trusted_db()?;
  let haystack: String = std::fs::read_to_string(&path).unwrap_or_default();
  let result: String = crate::editor::editor(true, &haystack, &format!(r"&1(.*)"), &format!(r"&1"), &[book_hash.0.clone()])?;
  if !result.is_empty() {
    if result == book_hash.1 { return Ok(true) }
    event!(Level::ERROR, "The recipe book is marked as trusted, but it's hash doesn't match\nExpected: {result}\nReceived: {}", book_hash.1);
    event!(Level::ERROR, "bh = {} {}", book_hash.0, book_hash.1);
  }
  display_book_script(temp_file);
  confirm_book_prompt(prompt)
}

pub fn trust(book: &String, text: &String) -> Res {
  let (book, hash) = hash(book, text)?;
  let path: std::path::PathBuf = crate::xdg::trusted_db()?;
  let haystack: String = std::fs::read_to_string(&path).unwrap_or_default();
  let result: String = crate::editor::editor(false, &haystack, &format!(r"&1.*\n"), &format!(""), &[book.clone()])?;
  let result: String = crate::editor::editor(false, &result, &format!(r"\z"), &format!(r"{book}{hash}\n"), &[])?;
  if haystack == result { return Ok(()) }
  gprl::fs::write_to_path(&path, &result)?;
  Ok(())
}
