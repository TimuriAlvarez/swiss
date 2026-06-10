use gprl::types::Res;

enum ReBuilder<'a> {
  Reference,
  Literals(&'a [String]),
  Free(&'a String),
  Word(&'a String),
  Line(&'a String),
}

impl<'a> ReBuilder<'a> {
  pub fn build(self) -> Result::<regex::Regex, regex::Error> {
    match self {
      ReBuilder::Reference => regex::Regex::new(r"&(\d+)"),
      ReBuilder::Literals(s) => regex::Regex::new(&format!(r"({})", s.join(r")\n("))),
      ReBuilder::Free(s) => regex::Regex::new(s),
      ReBuilder::Word(s) => regex::Regex::new(&format!(r"\<{s}\>")),
      ReBuilder::Line(s) => regex::Regex::new(&format!(r"(?m)^{s}$")),
    }
  }
}

fn replace_all(re: &regex::Regex, haystack: &str, f: impl Fn(&regex::Captures) -> Res::<String>) -> Res::<String> {
  let mut new: String = String::new();
  let mut last_match: usize = 0;
  for caps in re.captures_iter(haystack) {
    let m: regex::Match = caps.get(0).unwrap();
    new.push_str(&haystack[last_match..m.start()]);
    new.push_str(&f(&caps)?);
    last_match = m.end();
  }
  new.push_str(&haystack[last_match..]);
  Ok(new)
}

fn expand_refs_caps(expression_haystack: &String, expression_caps: &regex::Captures, escape: bool) -> Res::<String> {
  // Create a regular expression of the reference
  let re: regex::Regex = ReBuilder::Reference.build()?;
  // Replace all occurrences of the reference pattern with the corresponding current capture
  Ok(replace_all(&re, expression_haystack, |caps: &regex::Captures| -> Res::<String> {
    // Escape the replacement if necessary
    let result: &str = &expression_caps[caps[1].parse::<usize>()?];
    Ok(if escape {
      regex::escape(result)
    } else {
      result.to_string()
    })
  })?)
}

fn expand_refs_values(expression_haystack: &String, literals: &[String]) -> Res::<String> {
  let haystack: String = literals.join("\n");
  let literals: Vec::<String> = literals.into_iter().map(|literals: &String| regex::escape(literals)).collect();
  let re: regex::Regex = ReBuilder::Literals(&literals).build()?;
  let expression_caps: regex::Captures = re.captures(&haystack).unwrap();
  expand_refs_caps(expression_haystack, &expression_caps, true)
}

pub fn editor(mode: &str, haystack: &String, pattern: &String, replacement: &String, literals: &[String]) -> Res::<String> {
  // Expand all references from the pattern to literals
  let pat: String = expand_refs_values(pattern, literals)?;
  // Create a regular expression from the pattern
  let re: regex::Regex = match mode {
    "Free" => ReBuilder::Free(&pat).build()?,
    "Word" => ReBuilder::Word(&pat).build()?,
    "Line" => ReBuilder::Line(&pat).build()?,
    _ => panic!("Invalid editor mode"),
  };
  // Unescape the replacement
  let rep: String = unescape::unescape(replacement).unwrap();
  // Replace all occurrences of the pattern with the replacement
  Ok(replace_all(&re, haystack, |caps: &regex::Captures| -> Res::<String> {
    // Expand all references from the replacement to current captures
    Ok(expand_refs_caps(&rep, caps, false)?)
  })?)
}
