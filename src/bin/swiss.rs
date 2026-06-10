use gprl::types::Res;
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
  /// Recipe book's name
  #[arg(allow_hyphen_values=true)]
  book: Option::<String>,
  /// Recipe's arguments
  #[arg(allow_hyphen_values=true)]
  args: Vec::<String>,
}

fn view(vm: &swiss::viewer::ViewModel) -> Res {
  event!(Level::INFO, "view(ViewModel)\n{vm:#?}");
  Ok(())
}

fn main() -> Res {
  let app: CLI = CLI::parse();
  tracing_subscriber::fmt().with_max_level(app.level).init();
  if app.args.is_empty() {
    view(&swiss::viewer::presenter(&app.book)?)
  } else {
    todo!("run")
  }
}
