use gprl::types::Res;

pub fn data_path<P: AsRef<std::path::Path>>(path: P) -> Result::<std::path::PathBuf, std::io::Error> {
  xdg::BaseDirectories::with_prefix(env!("CARGO_PKG_NAME")).create_data_directory(path)
}

use lexical_sort::StringSort;

pub const EXTENSION: &str = "justfile";

pub fn books() -> Res::<Vec::<String>> {
  let entries: Vec::<dirwalk::Entry> = dirwalk::WalkBuilder::new(data_path("books")?).build()?.entries;
  let mut books: Vec::<String> = vec![];
  for entry in entries {
    if entry.extension() == Some(EXTENSION) {
      books.push(entry.relative_path[..entry.relative_path.len()-EXTENSION.len()-1].to_string());
    }
  }
  books.string_sort(lexical_sort::natural_lexical_cmp);
  Ok(books)
}
