use gprl::types::Res;
use std::io::Write;
use tracing::{event, Level};

fn confirm_book_execution() -> Res::<bool> {
  print!(":: Proceed with running the book? [Y/n] ");
  std::io::stdout().flush()?;
  let mut input: String = String::new();
  std::io::stdin().read_line(&mut input)?;
  input = input.trim().to_lowercase();
  Ok(input == "" || input == "y")
}

pub fn confirm(book: &String, text: &String) -> Res::<bool> {
  event!(Level::WARN, ":: trusted-agent is running {book:?} book:\n{text}");
  confirm_book_execution()
}
