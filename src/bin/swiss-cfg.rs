use clap::{Parser, Subcommand};

/// 🐺 Just a configuration editor - https://github.com/TimuriAlvarez/swiss
#[derive(Parser)]
#[command(version)]
pub struct CLI {
  /// Log level
  #[arg(long="log-level", default_value="info")]
  level: tracing::Level,
  #[command(subcommand)]
  command: Action,
}

#[derive(Subcommand)]
enum Action {
  /// Mark a recipe book as trusted
  Trust {
    /// Recipe book's name
    book: String
  },
}

fn main() -> gprl::types::Res {
  let app: CLI = CLI::parse();
  tracing_subscriber::fmt().with_max_level(app.level).init();
  match app.command {
    Action::Trust { book } => {
      let text: String = swiss::xdg::book(&book)?;
      let temp_file: temp_file::TempFile = temp_file::TempFile::with_suffix(".justfile")?.with_contents(&text.clone().into_bytes())?;
      if !swiss::trusted_agent::confirm(&book, &text, &temp_file, swiss::trusted_agent::PROMPT)? { return Ok(()) }
      swiss::trusted_agent::trust(&book, &text)?;
    },
  }
  Ok(())
}
