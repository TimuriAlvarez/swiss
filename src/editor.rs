use gprl::types::Res;
use tracing::{event, Level};

enum ReBuilder<'a> {
  Literals(&'a [String]),
  Pattern(&'a String),
}

impl<'a> ReBuilder<'a> {
  const REFERENCE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(||
    regex::Regex::new(r"(&?)&([[:digit:]]+):?").expect("Failed to construct a regex")
  );
  pub fn build(self) -> Result<regex::Regex, regex::Error> {
    match self {
      ReBuilder::Literals(s) => regex::Regex::new(&format!(r"({})", s.join(r")\n("))),
      ReBuilder::Pattern(s) => regex::Regex::new(&format!(r"(?m){s}")),
    }
  }
}

fn replace_all(extract: bool, re: &regex::Regex, haystack: &str, f: impl Fn(&regex::Captures) -> Res<String>) -> Res<String> {
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

fn expand_refs_caps(expression_haystack: &str, expression_caps: &regex::Captures, escape: bool) -> Res<String> {
  // Replace all occurrences of the reference pattern with the corresponding current capture
  Ok(replace_all(false, &ReBuilder::REFERENCE, expression_haystack, |caps: &regex::Captures| -> Res<String> {
    // Fetch prepended state and index value
    let prepended: bool = caps[1].len() > 0usize;
    let index: usize = caps[2].parse::<usize>()?;
    let result: String = if prepended {
      // Trim prefix and suffix
      format!("&{index}")
    } else {
      // Dereference index
      expression_caps[index].to_string()
    };
    Ok(if escape {
      // If the result was obtained from a literal: escape it
      regex::escape(&result)
    } else {
      // Else: leave it as is to prevent unescaping strings that already were unescaped
      result.to_string()
    })
  })?)
}

fn expand_refs_values(expression_haystack: &str, literals: &[String]) -> Res<String> {
  // Bundle literals into a haystack
  let haystack: String = literals.join("\n");
  // Build a special regex for these literals
  let literals: Vec<String> = literals.into_iter().map(|literals: &String| regex::escape(literals)).collect();
  let re: regex::Regex = ReBuilder::Literals(&literals).build()?;
  // Convert literals into captures
  let expression_caps: regex::Captures = re.captures(&haystack).unwrap();
  // Expand all references from expression haystack to literals' captures
  expand_refs_caps(expression_haystack, &expression_caps, true)
}

pub fn editor(extract: bool, haystack: &str, pattern: &str, replacement: &str, literals: &[String]) -> Res<String> {
  // Expand all references from the pattern to literals
  let pat: String = expand_refs_values(pattern, literals)?;
  event!(Level::DEBUG, "pattern = {pat:?}");
  // Create a regular expression from the pattern
  let re: regex::Regex = ReBuilder::Pattern(&pat).build()?;
  event!(Level::DEBUG, "re = {re:?}");
  // Unescape the replacement
  let rep: String = unescape::unescape(replacement).expect(&format!("Failed to unescape {replacement:?} string"));
  event!(Level::DEBUG, "replacement = {rep:?}");
  // Replace all occurrences of the pattern with the replacement
  Ok(replace_all(extract, &re, haystack, |caps: &regex::Captures| -> Res<String> {
    // Expand all references from the replacement to current captures
    event!(Level::DEBUG, "captures = {caps:?}");
    Ok(expand_refs_caps(&rep, caps, false)?)
  })?)
}
