const SII_KEY: &str = "SWISS_INSTANCE_ID";

pub fn data_path(path: &str) -> std::io::Result::<std::path::PathBuf> {
  xdg::BaseDirectories::with_prefix(env!("CARGO_PKG_NAME")).create_data_directory(path)
}

pub fn book(book: &String) -> std::io::Result::<String> {
  std::fs::read_to_string(data_path("books")?.join(format!("{book}.justfile")))
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

pub fn cache_path(global: bool, sii: Option::<&str>) -> std::io::Result::<std::path::PathBuf> {
  let xdg: xdg::BaseDirectories = xdg::BaseDirectories::with_prefix(env!("CARGO_PKG_NAME"));
  let path: &str = if global {
    "variables"
  } else if let Some(sii) = sii {
    sii
  } else {
    &std::env::var(SII_KEY).expect("environment variable SWISS_INSTANCE_ID")
  };
  xdg.create_cache_directory(path)
}

pub fn trusted_db() -> std::io::Result::<std::path::PathBuf> {
  xdg::BaseDirectories::with_prefix(env!("CARGO_PKG_NAME")).place_config_file("trusted.db")
}
