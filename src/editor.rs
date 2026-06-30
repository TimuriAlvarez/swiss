use gprl::types::Res;
use tracing::{event, Level};

enum ReBuilder<'a> {
  Reference,
  Literals(&'a [String]),
  Pattern(&'a String),
}

impl<'a> ReBuilder<'a> {
  const RESERVED: [&'static str; 10] = [r"\n", r"\r", r"\A", r"\z", r"\b", r"\B", r"\<", r"\>", r"^", r"$"];
  fn reserved_prefix(s: &String) -> bool {
    Self::RESERVED.into_iter().any(|pat: &str| s.starts_with(pat))
  }
  fn reserved_suffix(s: &String) -> bool {
    Self::RESERVED.into_iter().any(|pat: &str| s.ends_with(pat))
  }
  pub fn build(self) -> Result::<regex::Regex, regex::Error> {
    match self {
      ReBuilder::Reference => regex::Regex::new(r"&(\d+)"),
      ReBuilder::Literals(s) => regex::Regex::new(&format!(r"({})", s.join(r")\n("))),
      ReBuilder::Pattern(s) => {
        let begin: &str = if Self::reserved_prefix(s) { "" } else { "^" };
        let end: &str = if Self::reserved_suffix(s) { "" } else { "$" };
        regex::Regex::new(&format!(r"(?m){begin}{s}{end}"))
      },
    }
  }
}

fn replace_all(extract: bool, re: &regex::Regex, haystack: &str, f: impl Fn(&regex::Captures) -> Res::<String>) -> Res::<String> {
  // Initialize empty string, set the text cursor to the very beginning
  let mut new: String = String::new();
  let mut last_match: usize = 0;
  // Iterate over all captures
  for caps in re.captures_iter(haystack) {
    let m: regex::Match = caps.get(0).unwrap();
    // If not extracting: preserve the original data before the first capture and in-between captures
    new.push_str(if extract { "" } else { &haystack[last_match..m.start()] });
    // Obtain a replacement for the current capture
    new.push_str(&f(&caps)?);
    // Set the text cursor after the match
    last_match = m.end();
  }
  // If not extracting: preserve the original data after the last capture
  new.push_str(if extract { "" } else { &haystack[last_match..] });
  Ok(new)
}

fn expand_refs_caps(expression_haystack: &String, expression_caps: &regex::Captures, escape: bool) -> Res::<String> {
  // Create a regular expression of the reference
  let re: regex::Regex = ReBuilder::Reference.build()?;
  // Replace all occurrences of the reference pattern with the corresponding current capture
  Ok(replace_all(false, &re, expression_haystack, |caps: &regex::Captures| -> Res::<String> {
    let result: &str = &expression_caps[caps[1].parse::<usize>()?];
    Ok(if escape {
      // If the result was obtained from a literal: escape it
      regex::escape(result)
    } else {
      // Else: leave it as is to prevent unescaping strings that already were unescaped
      result.to_string()
    })
  })?)
}

fn expand_refs_values(expression_haystack: &String, literals: &[String]) -> Res::<String> {
  // Bundle literals into a haystack
  let haystack: String = literals.join("\n");
  // Build a special regex for these literals
  let literals: Vec::<String> = literals.into_iter().map(|literals: &String| regex::escape(literals)).collect();
  let re: regex::Regex = ReBuilder::Literals(&literals).build()?;
  // Convert literals into captures
  let expression_caps: regex::Captures = re.captures(&haystack).unwrap();
  // Expand all references from expression haystack to literals' captures
  expand_refs_caps(expression_haystack, &expression_caps, true)
}

pub fn editor(extract: bool, haystack: &String, pattern: &String, replacement: &String, literals: &[String]) -> Res::<String> {
  // Expand all references from the pattern to literals
  let pat: String = expand_refs_values(pattern, literals)?;
  event!(Level::DEBUG, "pattern = {pat:?}");
  // Create a regular expression from the pattern
  let re: regex::Regex = ReBuilder::Pattern(&pat).build()?;
  event!(Level::DEBUG, "re = {re:?}");
  // Unescape the replacement
  let rep: String = unescape::unescape(replacement).unwrap();
  event!(Level::DEBUG, "replacement = {rep:?}");
  // Replace all occurrences of the pattern with the replacement
  Ok(replace_all(extract, &re, haystack, |caps: &regex::Captures| -> Res::<String> {
    // Expand all references from the replacement to current captures
    event!(Level::DEBUG, "captures = {caps:?}");
    Ok(expand_refs_caps(&rep, caps, false)?)
  })?)
}
