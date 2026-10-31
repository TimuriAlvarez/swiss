use clap::Parser;

/// 🐺 Just a script manager - https://github.com/TimuriAlvarez/swiss
#[derive(Parser)]
#[command(version)]
pub struct CLI {
  /// Log level
  #[arg(long="log-level", default_value="info")]
  filter: tracing::Level,
  /// Recipe book's name or path
  book: Option<String>,
  /// Recipe's arguments
  #[arg(allow_hyphen_values=true)]
  args: Vec<String>,
}

fn main() -> gprl::types::Res {
  let app: CLI = CLI::parse();
  tracing_subscriber::fmt().with_max_level(app.filter).init();
  let default: bool = swiss::runner::viewer(&app.book)?;
  if let Some(book) = app.book {
    if !default && app.args.is_empty() { return Ok(()) }
    let args: Vec<&str> = app.args.iter().map(String::as_str).collect();
    swiss::runner::runner(&book, &args, None)?;
  }
  Ok(())
}
