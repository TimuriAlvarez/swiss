use gprl::types::Res;
use clap::{Parser, ValueEnum};
use tracing::{event, Level};
use swiss::runner::{Process, Program};

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

fn view(vm: &swiss::viewer::ViewModel) -> Res {
  event!(Level::INFO, "view(ViewModel)\n{vm:#?}");
  Ok(())
}

fn run(book: &String, args: &[String]) -> Res {
  let text: String = swiss::xdg::book(book)?;
  if !swiss::trust_agent::confirm(book, &text)? {
    return Ok(())
  }
  let text: String = swiss::xdg::expand(&text);
  let temp_file: temp_file::TempFile = temp_file::with_contents(&text.into_bytes());
  Process::new(Program::Just, &["--justfile"], Some(&temp_file)).spawn(args)?;
  Ok(())
}

fn main() -> Res {
  let app: CLI = CLI::parse();
  tracing_subscriber::fmt().with_max_level(app.level).init();
  if app.args.is_empty() {
    view(&swiss::viewer::presenter(&app.book)?)
  } else {
    run(&app.book.expect("Book is missing"), &app.args)
  }
}
