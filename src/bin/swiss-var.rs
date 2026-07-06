use clap::Parser;

/// 🐺 Just a variable manager - https://github.com/TimuriAlvarez/swiss
#[derive(Parser)]
#[command(version)]
pub struct CLI {
  /// Log level
  #[arg(long="log-level", default_value="info")]
  filter: tracing::Level,
  /// Manage global variables
  #[arg(long)]
  global: bool,
  /// Command
  #[command(subcommand)]
  command: Subcommand,
}

#[derive(clap::Subcommand)]
enum Subcommand {
  /// Get a variable's value
  Get {
    name: String,
  },
  /// Set a variable's value
  Set {
    /// Variable's name
    name: String,
    /// Variables's value
    #[arg(allow_hyphen_values=true)]
    value: String,
  },
}

fn main() -> swiss::Res {
  let app: CLI = CLI::parse();
  tracing_subscriber::fmt().with_max_level(app.filter).init();
  match app.command {
    Subcommand::Get { name } => swiss::variable::get(app.global, &name)?,
    Subcommand::Set { name, value } => swiss::variable::set(app.global, &name, &value)?,
  }
  Ok(())
}
