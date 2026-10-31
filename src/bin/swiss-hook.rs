use clap::Parser;

/// 🐺 Just a hook manager - https://github.com/TimuriAlvarez/swiss
#[derive(Parser)]
#[command(version)]
pub struct CLI {
  /// Log level
  #[arg(long="log-level", default_value="info")]
  filter: tracing::Level,
  /// Instance signature
  signature: String,
  /// Output variable
  variable: String,
  /// Recipe book's name or path
  book: String,
  /// Entry's name
  entry: String,
}

fn main() -> gprl::types::Res {
  let app: CLI = CLI::parse();
  tracing_subscriber::fmt().with_max_level(app.filter).init();
  let hook: swiss::preprocessor::prelude::Hook = swiss::preprocessor::prelude::Hook {
    signature : app.signature,
    variable: app.variable,
    entry: app.entry,
  };
  swiss::runner::runner(&app.book, &["swiss-hook"], Some(&hook))?;
  Ok(())
}
