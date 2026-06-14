use gprl::types::Res;
use clap::{Parser, ValueEnum};
use swiss::runner::{spawn, Program::*};

#[derive(ValueEnum, Clone, derive_more::Display)]
pub enum EditorMode {
  Free,
  Word,
  Line,
}

/// 🐺 Just a script manager - https://github.com/TimuriAlvarez/swiss
#[derive(Parser)]
#[command(version)]
pub struct CLI {
  /// Log level
  #[arg(long="log-level", default_value="info")]
  filter: tracing::Level,
  /// Recipe book's name
  #[arg(allow_hyphen_values=true)]
  book: Option::<String>,
  /// Recipe's arguments
  #[arg(allow_hyphen_values=true)]
  args: Vec::<String>,
}

fn run(book: &String, args: &[String]) -> Res {
  let text: String = swiss::xdg::book(book)?;
  let temp_file: temp_file::TempFile = temp_file::TempFile::with_suffix(".justfile")?.with_contents(&text.into_bytes())?;
  if !swiss::trusted_agent::confirm(book, &temp_file)? { return Ok(()) }
  let text: String = std::fs::read_to_string(&temp_file)?;
  let text: String = swiss::xdg::expand(&text);
  gprl::fs::write_to_path(temp_file.path(), &text)?;
  swiss::runner::runner(spawn, Just, &["--justfile"], Some(&temp_file), args).map(|_| ())
}

fn main() -> Res {
  let app: CLI = CLI::parse();
  tracing_subscriber::fmt().with_max_level(app.filter).init();
  swiss::viewer::viewer(&app.book)?;
  if !app.args.is_empty() {
    run(&app.book.expect("Book is missing"), &app.args)?;
  }
  Ok(())
}
