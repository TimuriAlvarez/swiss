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
  let list: &str = include_str!("../resources/list.justfile").trim();
  let literal: &str = include_str!("../resources/literal.justfile").trim();
  let variable: &str = include_str!("../resources/variable.justfile").trim();
  let default: &str = include_str!("../resources/default.justfile").trim();
  let variable: String = variable.replace("%%", SII_KEY);
  let mut parts: Vec::<&str> = vec![text, common, list, literal, &variable];
  if !text.lines().any(|line: &str| line == "[default]") {
    parts.push(default);
  }
  format!("{}\n", parts.join("\n\n"))
}

fn cache_path(global: bool, swiss_instance_id: Option::<&str>) -> std::io::Result::<std::path::PathBuf> {
  let xdg: xdg::BaseDirectories = xdg::BaseDirectories::with_prefix(env!("CARGO_PKG_NAME"));
  let path: &str = if global {
    "variables"
  } else if let Some(swiss_instance_id) = swiss_instance_id {
    swiss_instance_id
  } else {
    &std::env::var(SII_KEY).expect("environment variable SWISS_INSTANCE_ID")
  };
  xdg.create_cache_directory(path)
}

pub fn var_get(global: bool, name: &String) -> Res {
  let value: String = std::fs::read_to_string(cache_path(global, None)?.join(name)).unwrap_or_default();
  print!("{value}");
  Ok(())
}

pub fn var_set(global: bool, name: &String, value: &String) -> Res {
  gprl::fs::write_to_path(cache_path(global, None)?.join(name), value)?;
  Ok(())
}

pub fn var_purge(swiss_instance_id: &str) -> Res {
  std::fs::remove_dir_all(cache_path(false, Some(swiss_instance_id))?)?;
  Ok(())
}

pub fn trusted_db() -> std::io::Result::<std::path::PathBuf> {
  xdg::BaseDirectories::with_prefix(env!("CARGO_PKG_NAME")).place_config_file("trusted.db")
}
