use gprl::types::Res;

mod viewer;
mod shell;
mod extensions;

fn books_path() -> std::io::Result<std::path::PathBuf> {
  xdg::BaseDirectories::with_prefix(env!("CARGO_PKG_NAME")).create_data_directory("books")
}

fn contents(book: &str) -> Res<String> {
  let local: std::path::PathBuf = std::path::PathBuf::from(book);
  let path: std::path::PathBuf = if local.exists() { local } else {
    books_path()?.join(&format!("{book}.just"))
  };
  let book: String = std::fs::read_to_string(path)?;
  let swiss: &str = include_str!("../../resources/swiss.just");
  Ok(format!("{book}\n{swiss}"))
}

fn list_books() -> Res<String> {
  use lexical_sort::StringSort;
  let entries: Vec<dirwalk::Entry> = dirwalk::WalkBuilder::new(books_path()?).build()?.entries;
  let mut books: Vec<String> = vec![];
  for entry in entries {
    if entry.extension() == Some("just") {
      books.push(entry.relative_path[..entry.relative_path.len()-"just".len()-1].to_string());
    }
  }
  books.string_sort(lexical_sort::natural_lexical_cmp);
  Ok(books.join("\n"))
}

pub fn viewer(book: &Option<String>) -> Res<bool> {
  let mut default: bool = false;
  let markdown: String = if let Some(book) = book {
    let contents: String = extensions::apply(&contents(book)?)?;
    default = contents.lines().any(|line: &str| line == "[default]");
    let tempfile: temp_file::TempFile = temp_file::with_contents(&contents.into_bytes());
    let recipes: shell::Output = shell::run(shell::output, shell::JUST, &["--list", "--list-heading", "", "--list-prefix", "", "--color", "always", "--justfile"], Some(&tempfile), &[])?;
    if !recipes.success {
      anyhow::bail!("unable to retrieve recipes from `{book}` book\n\n{}", recipes.stderr)
    }
    viewer::book(book, &recipes.stdout)?
  } else {
    viewer::app(&list_books()?)?
  };
  let tempfile: temp_file::TempFile = temp_file::with_contents(&markdown.into_bytes());
  shell::run(shell::spawn, shell::GLOW, &["--width", "0"], Some(&tempfile), &[]).map(|_| default)
}

pub fn runner(book: &str, args: &[String]) -> Res {
  let contents: String = extensions::apply(&contents(book)?)?;
  let tempfile: temp_file::TempFile = temp_file::with_contents(&contents.into_bytes());
  let result: Res = shell::run(shell::spawn, shell::JUST, &["--justfile"], Some(&tempfile), args).map(|_| ());
  let signature: &str = tempfile.path().file_stem().expect("stem extraction failure").to_str().expect("path conversion failure");
  crate::variable::purge(signature)?;
  result
}
