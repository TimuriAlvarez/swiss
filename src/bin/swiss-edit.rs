use clap::{Parser, ValueEnum};
use tracing::{event, Level};

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
  /// Editor mode
  #[arg(short, long, default_value="line")]
  mode: EditorMode,
  /// Path to the file to be edited
  #[arg(allow_hyphen_values=true)]
  file: String,
  /// A regex pattern to look for
  #[arg(allow_hyphen_values=true)]
  pattern: String,
  /// A replacement for the pattern's matches
  #[arg(allow_hyphen_values=true)]
  replacement: String,
  /// String literals (escaped before being fed to the pattern)
  #[arg(allow_hyphen_values=true)]
  literals: Vec::<String>,
}

fn main() -> gprl::types::Res {
  let app: CLI = CLI::parse();
  tracing_subscriber::fmt().with_max_level(app.level).init();
  let haystack: String = std::fs::read_to_string(&app.file)?;
  let result: String = swiss::editor::editor(&app.mode.to_string(), &haystack, &app.pattern, &app.replacement, &app.literals)?;
  if haystack == result {
    event!(Level::INFO, "No changes to the '{}' file were made", app.file);
  } else {
    gprl::fs::write_to_path(app.file, &result)?;
  }
  Ok(())
}
