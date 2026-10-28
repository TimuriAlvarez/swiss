use clap::Parser;

/// 🐺 Just a variable manager - https://github.com/TimuriAlvarez/swiss
#[derive(Parser)]
#[command(version)]
pub struct CLI {
  /// Log level
  #[arg(long="log-level", default_value="info")]
  filter: tracing::Level,
  /// Variable's persistence
  #[arg(long="persist")]
  persist: bool,
  /// Instance signature
  signature: String,
  /// Command
  #[command(subcommand)]
  command: Subcommand,
}

#[derive(clap::Subcommand)]
enum Subcommand {
  /// Set variable's value
  Set {
    /// Variable's name
    name: String,
    /// Variable's value
    #[arg(allow_hyphen_values=true)]
    value: Vec<String>,
  },
  /// Get next index
  Next {
    /// Variable's name
    name: String,
    /// Current index
    index: Option<usize>,
  },
  /// Get variable's value
  Get {
    /// Variable's name
    name: String,
    /// Partial index
    index: usize,
  },
  /// Purge all variables associated with the instance signature
  Purge,
}

fn main() -> gprl::types::Res {
  let app: CLI = CLI::parse();
  tracing_subscriber::fmt().with_max_level(app.filter).init();
  match app.command {
    Subcommand::Set { name, value: values } => swiss::variables::set(app.persist, &app.signature, &name, &values),
    Subcommand::Next { name, index: current } => swiss::variables::next(app.persist, &app.signature, &name, current),
    Subcommand::Get { name, index } => swiss::variables::get(app.persist, &app.signature, &name, index),
    Subcommand::Purge => swiss::variables::purge(app.persist, &app.signature),
  }
}
