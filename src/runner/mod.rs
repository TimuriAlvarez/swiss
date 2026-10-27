use gprl::types::Res;

mod markdown;
mod shell;

const GLOW: &str = "glow";
const JUST: &str = "just";

fn books_path() -> std::io::Result<std::path::PathBuf> {
  xdg::BaseDirectories::with_prefix(env!("CARGO_PKG_NAME")).create_data_directory("books")
}

fn contents(book: &str) -> Res<String> {
  let local: std::path::PathBuf = std::path::PathBuf::from(book);
  let path: std::path::PathBuf = if local.exists() { local } else {
    books_path()?.join(&format!("{book}.just"))
  };
  let contents: String = std::fs::read_to_string(path)?;
  crate::preprocessor::run(&contents)
}

fn list_recipes(book: &String, contents: String) -> Res<String> {
  let tempfile: temp_file::TempFile = temp_file::with_contents(&contents.into_bytes());
  let all_args: Vec<&str> = vec!["--justfile", tempfile.path().to_str().expect("path conversion failed"), "--list-heading", "", "--list-prefix", "", "--list"];
  let state: shell::State = shell::runner(shell::output, JUST, &all_args)?;
  if !state.status.success() {
    anyhow::bail!("unable to retrieve recipes from `{book}` book\n\n{}", state)
  }
  Ok(state.stdout)
}

fn list_books() -> Res<Vec<String>> {
  use lexical_sort::StringSort;
  let entries: Vec<dirwalk::Entry> = dirwalk::WalkBuilder::new(books_path()?).build()?.entries;
  let mut books: Vec<String> = vec![];
  for entry in entries {
    if entry.extension() == Some("just") {
      books.push(entry.relative_path[..entry.relative_path.len()-"just".len()-1].to_string());
    }
  }
  books.string_sort(lexical_sort::natural_lexical_cmp);
  Ok(books)
}

pub fn viewer(book: &Option<String>) -> Res<bool> {
  let mut default: bool = false;
  let contents: String = if let Some(book) = book {
    let contents: String = contents(book)?;
    default = crate::preprocessor::has_attribute(&contents, "default")?;
    markdown::book(book, &list_recipes(book, contents)?)
  } else {
    markdown::default(&list_books()?)
  }?;
  let tempfile: temp_file::TempFile = temp_file::with_contents(&contents.into_bytes());
  if shell::runner(shell::spawn, GLOW, &["--width", "0", tempfile.path().to_str().expect("path conversion failed")]).is_err() {
    println!("{}", std::fs::read_to_string(tempfile.path())?);
  }
  Ok(default)
}

pub fn runner(book: &String, args: &[String]) -> Res {
  let contents: String = contents(book)?;
  let tempfile: temp_file::TempFile = temp_file::with_contents(&contents.into_bytes());
  let mut all_args: Vec<&str> = vec!["--justfile", tempfile.path().to_str().expect("path conversion failed")];
  all_args.extend(args.iter().map(|arg: &String| arg.as_str()));
  let state: shell::State = shell::runner(shell::spawn, JUST, &all_args)?;
  let signature: &str = tempfile.path().file_stem().expect("stem extraction failure").to_str().expect("path conversion failure");
  crate::variables::purge(signature)?;
  shell::propagate(state)
}
