use gprl::types::Res;
use clap::{Parser, ValueEnum};
use swiss::runner::{spawn, Program::*};

#[derive(ValueEnum, Clone, derive_more::Display)]
pub enum EditorMode {
  Free,
  Word,
  Line,
}

/// Edit a specified file's content (powered by 'regex')
#[derive(Parser)]
#[command(version, about)]
pub struct CLI {
  /// Log level
  #[arg(long="log-level", default_value="info")]
  level: tracing::Level,
  /// Recipe book's name
  #[arg(allow_hyphen_values=true)]
  book: Option::<String>,
  /// Recipe's arguments
  #[arg(allow_hyphen_values=true)]
  args: Vec::<String>,
}

fn run(book: &String, args: &[String]) -> Res {
  let text: String = swiss::xdg::book(book)?;
  if !swiss::trusted_agent::confirm(book, &text)? {
    return Ok(())
  }
  let text: String = swiss::xdg::expand(&text);
  swiss::runner::runner(spawn, Just, &["--justfile"], Some(&text), args).map(|_| ())
}

fn main() -> Res {
  let app: CLI = CLI::parse();
  tracing_subscriber::fmt().with_max_level(app.level).init();
  swiss::viewer::viewer(&app.book)?;
  if !app.args.is_empty() {
    run(&app.book.expect("Book is missing"), &app.args)?;
  }
  Ok(())
}
