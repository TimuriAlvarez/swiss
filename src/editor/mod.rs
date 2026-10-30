use descape::UnescapeExt;
use gprl::types::Res;

mod prelude;

pub use prelude::Config;
use prelude::ReBuilder;

fn replace_all(config: &Config, re: &regex::Regex, haystack: &str, f: impl Fn(&regex::Captures) -> Res<String>) -> Res<String> {
  // Initialize empty string, set the text cursor to the very beginning
  let mut result: String = String::new();
  let mut last_match: usize = 0;
  // Iterate over all captures
  for caps in re.captures_iter(haystack) {
    let m: regex::Match = caps.get(0).unwrap();
    // If not extracting: preserve the original data before the first capture and in-between captures
    result.push_str(if config.extract { "" } else { &haystack[last_match..m.start()] });
    // Obtain a replacement for the current capture
    result.push_str(&f(&caps)?);
    // Set the text cursor after the match
    last_match = m.end();
  }
  // If not extracting: preserve the original data after the last capture
  result.push_str(if config.extract { "" } else { &haystack[last_match..] });
  Ok(result)
}

fn expand_refs_caps(re: &regex::Regex, escape: &str, expression_haystack: &str, expression_caps: &regex::Captures, f: impl Fn(&str) -> String) -> Res<String> {
  // Replace all occurrences of the reference pattern with the corresponding current capture
  replace_all(&Config::DEFAULT, re, expression_haystack, |captures: &regex::Captures| -> Res<String> {
    // Fetch group's key
    let mut key: &str = &captures[1];
    if key.starts_with('{') && key.ends_with('}') {
      key = &key[1..key.len() - 1];
    }
    // Fetch group's value
    let value: &str = if key == escape { key }
    else if let Ok(index) = key.parse::<usize>() { &expression_caps[index] }
    else { &expression_caps[key] };
    // Convert group's value
    Ok(f(value))
  })
}

fn expand_refs_vals(expression_haystack: &str, literals: &[&str], f: impl Fn(&str) -> String) -> Res<String> {
  // Bundle literals into a haystack
  let haystack: String = literals.join("\x00");
  // Build a special regex for these literals
  let literals: Vec<String> = literals.into_iter().map(|literal: &&str| f(literal)).collect();
  let re: regex::Regex = ReBuilder::Literals(&literals).build()?;
  // Convert literals into captures
  let expression_caps: regex::Captures = re.captures(&haystack).unwrap();
  // Expand all references from expression haystack to literals' captures
  expand_refs_caps(&ReBuilder::GROUP_SHIELD, ReBuilder::ESCAPE_SHIELD, expression_haystack, &expression_caps, f)
}

pub fn edit(config: &Config, haystack: &str, pattern: &str, replacement: &str, escape: &[&str], shield: &[&str]) -> Res<String> {
  // Unescape the replacement
  let replacement: String = replacement.to_unescaped().expect(&format!("Failed to unescape {replacement:?} string")).to_string();
  // Expand all references from the pattern to the corresponding literals from `escape`
  let pattern: String = expand_refs_vals(pattern, escape, regex::escape)?;
  // Expand all references from the replacement to the corresponding literals from `shield`
  let replacement: String = expand_refs_vals(&replacement, shield, |literal: &str| literal.replace('$', "$$"))?;
  // Create a regular expression from the pattern
  let re: regex::Regex = ReBuilder::Pattern(&pattern).build()?;
  // Replace all occurrences of the pattern in the haystack with the replacement
  replace_all(config, &re, haystack, |captures: &regex::Captures| -> Res<String> {
    // Expand all references from the replacement to current captures
    expand_refs_caps(&ReBuilder::GROUP_REGEX, ReBuilder::ESCAPE_REGEX, &replacement, captures, str::to_owned)
  })
}
