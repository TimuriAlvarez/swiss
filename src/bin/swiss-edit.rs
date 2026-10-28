use std::io::IsTerminal;

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
  /// If the requested file is not present or it is empty, the operation is skipped
  #[arg(long="skip-empty", default_value="false")]
  skip_empty: bool,
  /// Write to the specified file ('-' for stdout)
  #[arg(long="output", short='o', allow_hyphen_values=true)]
  output: Option<String>,
  /// Path to the file to be edited ('-' for stdin)
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
  literals: Vec<String>,
}

fn read_to_string(file: &str) -> std::io::Result<String> {
  if file != "-" { return Ok(std::fs::read_to_string(file).unwrap_or_default()) }
  let reader: std::io::Stdin = std::io::stdin();
  if reader.is_terminal() { return Ok(String::new()) }
  std::io::read_to_string(reader)
}

fn main() -> gprl::types::Res {
  let app: CLI = CLI::parse();
  tracing_subscriber::fmt().with_max_level(app.level).init();
  if app.skip_existing && std::path::Path::new(&app.file).exists() { return Ok(()) }
  let haystack: String = read_to_string(&app.file)?;
  if app.skip_empty && haystack.is_empty() { return Ok(()) }
  let literals: Vec<&str> = app.literals.iter().map(String::as_str).collect();
  let contents: String = swiss::editor::editor(app.extract, &haystack, &app.pattern, &app.replacement, &literals)?;
  let path: String = app.output.unwrap_or(app.file.clone());
  if path == "-" {
    print!("{contents}");
  } else if path == app.file && haystack == contents {
    event!(Level::INFO, "No changes to the '{}' file were made", app.file);
  } else {
    gprl::fs::write_to_path(path, &contents)?;
  }
  Ok(())
}
