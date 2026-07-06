use crate::Res;

mod viewer;
mod shell;

fn contents(book: &str) -> Res::<String> {
  let local: std::path::PathBuf = std::path::PathBuf::from(book);
  let path: std::path::PathBuf = if local.exists() { local } else {
    xdg::BaseDirectories::with_prefix(env!("CARGO_PKG_NAME")).create_data_directory("books")?.join(&format!("{book}.just"))
  };
  let book: String = std::fs::read_to_string(path)?;
  let swiss: &str = include_str!("../../resources/swiss.just");
  Ok(format!("{book}\n{swiss}"))
}

pub fn viewer(book: &Option::<String>) -> Res::<bool> {
  let mut default: bool = false;
  let markdown: String = if let Some(book) = book {
    let contents: String = contents(book)?;
    default = contents.lines().any(|line: &str| line == "[default]");
    let tempfile: temp_file::TempFile = temp_file::with_contents(&contents.into_bytes());
    let recipes: shell::Output = shell::run(shell::output, shell::JUST, &["--list", "--list-heading", "", "--list-prefix", "", "--color", "always", "--justfile"], Some(&tempfile), &[])?;
    if !recipes.success {
      anyhow::bail!("unable to retrieve recipes from `{book}` book\n\n{}", recipes.stderr)
    }
    viewer::BookViewModel::new(book, &recipes.stdout).to_string()
  } else {
    viewer::AppViewModel::new().to_string()
  };
  let tempfile: temp_file::TempFile = temp_file::with_contents(&markdown.into_bytes());
  shell::run(shell::spawn, shell::GLOW, &["--width", "0"], Some(&tempfile), &[]).map(|_| default)
}

pub fn runner(book: &str, args: &[String]) -> Res {
  let tempfile: temp_file::TempFile = temp_file::with_contents(&contents(book)?.into_bytes());
  shell::run(shell::spawn, shell::JUST, &["--justfile"], Some(&tempfile), args).map(|_| ())
}
