use gprl::types::Res;
use clap::Parser;
use swiss::runner::{spawn, Program::*};

/// 🐺 Just a script manager - https://github.com/TimuriAlvarez/swiss
#[derive(Parser)]
#[command(version)]
pub struct CLI {
  /// Log level
  #[arg(long="log-level", default_value="info")]
  filter: tracing::Level,
  /// Recipe book's name
  book: Option::<String>,
  /// Recipe's arguments
  #[arg(allow_hyphen_values=true)]
  args: Vec::<String>,
}

fn run(book: &String, args: &[String]) -> Res {
  let original_text: String = swiss::xdg::book(book)?;
  let temp_file: temp_file::TempFile = temp_file::TempFile::with_suffix(".justfile")?.with_contents(&original_text.clone().into_bytes())?;
  if !swiss::trusted_agent::confirm(book, &original_text, &temp_file, "Do you want to run this recipe book")? { return Ok(()) }
  let text: String = std::fs::read_to_string(&temp_file)?;
  let text: String = swiss::xdg::expand(&text);
  gprl::fs::write_to_path(temp_file.path(), &text)?;
  swiss::runner::runner(spawn, Just, &["--justfile"], Some(&temp_file), args)?;
  swiss::variable::purge(temp_file.path().file_stem().expect("Failed to parse file stem").to_str().expect("Failed to convert OsStr -> &str"))?;
  if !swiss::trusted_agent::confirm(book, &original_text, &temp_file, swiss::trusted_agent::PROMPT)? { return Ok(()) }
  swiss::trusted_agent::trust(book, &original_text)?;
  Ok(())
}

fn main() -> Res {
  let app: CLI = CLI::parse();
  tracing_subscriber::fmt().with_max_level(app.filter).init();
  swiss::viewer::viewer(&app.book)?;
  if let Some(book) = app.book {
    run(&book, &app.args)?;
  }
  Ok(())
}
