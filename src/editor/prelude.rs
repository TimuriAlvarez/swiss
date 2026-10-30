#[derive(Debug, Default)]
pub struct Config {
  pub extract: bool,
}

impl Config {
  pub const DEFAULT: std::sync::LazyLock<Self> = std::sync::LazyLock::new(|| Self::default() );
  pub const EXTRACT: std::sync::LazyLock<Self> = std::sync::LazyLock::new(|| Self::default().extract(true) );

  pub fn extract(mut self, extract: bool) -> Self {
    self.extract = extract;
    self
  }
}

pub enum ReBuilder<'a> {
  Group(&'a str),
  Pattern(&'a str),
  Literals(&'a [String]),
}

impl<'a> ReBuilder<'a> {
  pub const ESCAPE_REGEX: &str = "$";
  pub const ESCAPE_SHIELD: &str = "&";

  pub const GROUP_REGEX: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| Self::Group(Self::ESCAPE_REGEX).build().expect("regex construction failed"));
  pub const GROUP_SHIELD: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| Self::Group(Self::ESCAPE_SHIELD).build().expect("regex construction failed"));

  pub fn build(self) -> Result<regex::Regex, regex::Error> {
    regex::Regex::new(&match self {
      ReBuilder::Group(s) => format!(r"\{s}(\{s}|\{{[[:word:]]+\}}|[[:word:]]+)"),
      ReBuilder::Pattern(s) => format!(r"(?m){s}"),
      ReBuilder::Literals(s) => format!(r"^({})$", vec![r"[^\x00]*"; s.len()].join(r")\x00(")),
    })
  }
}
