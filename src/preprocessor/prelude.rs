use gprl::types::Res;

pub const ANY: char = '.';
pub const MARKER: char = '=';

pub const CHAR: &str = r"-|[[:word:]]";
pub const IDENT: &str = r"(?:-|[[:word:]])+";
pub const BLANK: &str = r"[[:blank:]]";
pub const OPERATOR: &str = r"\+\+|\+|/";
pub const EXPRESSION: &str = r#"\(.*\)|\[.*\]|"[^"]*"|'[^']*'|[^,\(\)\s]+"#;

pub type VarNames = std::collections::BTreeSet<String>;
pub type Variables = std::collections::BTreeMap<String, Option<String>>;

pub fn process_invalid_names(message: &str, names: VarNames) -> Res {
  if names.is_empty() { return Ok(()) }
  anyhow::bail!("{message}: {}", names.into_iter().collect::<Vec<String>>().join(", "))
}

pub struct Settings {
  pub vars: Variables,
}

impl Settings {
  pub const ALL: [&str; 1] = ["vars"];
}
