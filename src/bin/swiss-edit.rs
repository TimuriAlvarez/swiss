use clap::Parser;
use tracing::{event, Level};

/// 🐺 Just a file editor (powered by regex) - https://github.com/TimuriAlvarez/swiss
#[derive(Parser)]
#[command(version)]
pub struct CLI {
  /// Log level
  #[arg(long="log-level", default_value="info")]
  level: tracing::Level,
  /// Enable matches extraction
  #[arg(long="extract", default_value="false")]
  extract: bool,
  /// If there is a file present when edition is requested, the operation is skipped
  #[arg(long="skip-existing", default_value="false")]
  skip_existing: bool,
  /// Path to the file to be edited
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
  if app.skip_existing && std::path::Path::new(&app.file).exists() { return Ok(()) }
  let haystack: String = std::fs::read_to_string(&app.file).unwrap_or_default();
  let result: String = swiss::editor::editor(app.extract, &haystack, &app.pattern, &app.replacement, &app.literals)?;
  if haystack == result {
    event!(Level::INFO, "No changes to the '{}' file were made", app.file);
  } else {
    gprl::fs::write_to_path(app.file, &result)?;
  }
  Ok(())
}
