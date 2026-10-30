pub use gprl::types::Res;
pub use crate::editor;

pub const ANY: char = '.';
pub const MARKER: char = '=';

pub const CHAR: &str = r"-|[[:word:]]";
pub const IDENT: &str = r"(?:-|[[:word:]])+";
pub const BLANK: &str = r"[[:blank:]]";
pub const OPERATOR: &str = r"\+\+|\+|/";
pub const EXPRESSION: &str = r#"\(.*\)|\[.*\]|"[^"]*"|'[^']*'|[^,\(\)\s]+"#;

pub const REGION_BEGIN_INTERPOL: &str = r"r#\{\{";
pub const REGION_BEGIN_SCRIPT: &str = r"r#\$\$";
pub const REGION_END_INTERPOL: &str = r"#\}\}";
pub const REGION_END_SCRIPT: &str = r"#\$\$";

pub type Entry = std::collections::BTreeSet<String>;
pub type Entries = std::collections::BTreeMap<String, Option<String>>;

pub fn process_invalid_entries(message: &str, names: &Entry) -> Res {
  if names.is_empty() { return Ok(()) }
  anyhow::bail!("{message}\n{names:#?}")
}

#[derive(Debug)]
pub struct Settings {
  pub vars: Entries,
}

impl Settings {
  pub const ALL: [&str; 1] = ["vars"];
}

#[derive(Debug)]
pub struct RegionLine {
  pub text: String,
  pub indent: String,
  pub is_border: bool,
  pub is_rightmost: bool,
}

#[derive(Debug, Default)]
pub struct RegionLines {
  pub echo: bool,
  pub lines: Vec<RegionLine>,
}

impl RegionLines {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn echo(&self, condition: bool) -> &'static str {
    if self.echo && condition { "@ " } else { "" }
  }
}
