use clap::Parser;

/// 🐺 Just a variable manager - https://github.com/TimuriAlvarez/swiss
#[derive(Parser)]
#[command(version)]
pub struct CLI {
  /// Log level
  #[arg(long="log-level", default_value="info")]
  filter: tracing::Level,
  /// Instance signature
  #[arg()]
  signature: String,
  /// Variable's name
  #[arg()]
  name: String,
  /// Command
  #[command(subcommand)]
  command: Subcommand,
}

#[derive(clap::Subcommand)]
enum Subcommand {
  /// Set variable's value
  Set {
    /// Variable's value
    #[arg(allow_hyphen_values=true)]
    value: Vec<String>,
  },
  /// Get next index
  Next {
    /// Current index
    index: Option<usize>,
  },
  /// Get variable's value
  Get {
    /// Partial index
    index: usize,
  }
}

fn main() -> gprl::types::Res {
  let app: CLI = CLI::parse();
  tracing_subscriber::fmt().with_max_level(app.filter).init();
  match app.command {
    Subcommand::Set { value: values } => swiss::variables::set(&app.signature, &app.name, &values),
    Subcommand::Next { index: current } => swiss::variables::next(&app.signature, &app.name, current),
    Subcommand::Get { index } => swiss::variables::get(&app.signature, &app.name, index),
  }
}
