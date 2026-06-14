use gprl::types::Res;

pub fn data_path<P: AsRef<std::path::Path>>(path: P) -> Result::<std::path::PathBuf, std::io::Error> {
  xdg::BaseDirectories::with_prefix(env!("CARGO_PKG_NAME")).create_data_directory(path)
}

use lexical_sort::StringSort;

pub const EXTENSION: &str = "justfile";

pub fn books() -> Res::<String> {
  let entries: Vec::<dirwalk::Entry> = dirwalk::WalkBuilder::new(data_path("books")?).build()?.entries;
  let mut books: Vec::<String> = vec![];
  for entry in entries {
    if entry.extension() == Some(EXTENSION) {
      books.push(entry.relative_path[..entry.relative_path.len()-EXTENSION.len()-1].to_string());
    }
  }
  books.string_sort(lexical_sort::natural_lexical_cmp);
  Ok(books.join("\n"))
}

pub fn book(book: &String) -> Result::<String, std::io::Error> {
  std::fs::read_to_string(data_path("books")?.join(format!("{book}.{EXTENSION}")))
}

pub fn expand(text: &String) -> String {
  let common: &str = include_str!("../resources/common.justfile").trim();
  let variables: &str = include_str!("../resources/variables.justfile").trim();
  let conditions: &str = include_str!("../resources/conditions.justfile").trim();
  let lists: &str = include_str!("../resources/lists.justfile").trim();
  let default: &str = include_str!("../resources/default.justfile").trim();
  let mut parts: Vec::<&str> = vec![text, common, variables, conditions, lists];
  if !text.lines().any(|line: &str| line == "[default]") {
    parts.push(default);
  }
  format!("{}\n", parts.join("\n\n"))
}
