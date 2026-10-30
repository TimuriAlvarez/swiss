use super::prelude::*;

fn regex_vars_declarations(haystack: &str) -> Res<String> {
  let pattern: String = format!(r#"^&1{BLANK}+&2{BLANK}+:={BLANK}+({EXPRESSION})(({BLANK}+&3{BLANK}+&4{BLANK}+({EXPRESSION}))?)$"#);
  let replacement: String = format!("$1{MARKER}$2\n");
  editor::edit(&editor::Config::EXTRACT, haystack, &pattern, &replacement, &["set", "swiss-vars", "persist", "as"], &[])
}

fn regex_vars_trail(haystack: &str) -> Res<String> {
  let pattern: String = format!(r#"^{BLANK}+&3{BLANK}+&4{BLANK}+({EXPRESSION})$"#);
  editor::edit(&editor::Config::EXTRACT, haystack, &pattern, "$1", &["set", "swiss-vars", "persist", "as"], &[])
}

fn regex_trim_trail(haystack: &str) -> Res<String> {
  let pattern: String = format!(r#"^['''|"""|'|"](.*?)['''|"""|'|"]$"#);
  editor::edit(&editor::Config::DEFAULT, haystack, &pattern, "$1", &[], &[])
}

fn load_entries(contents: &str, declarations: impl Fn(&str) -> Res<String>, trail: impl Fn(&str) -> Res<String>, name: &str) -> Res<Entries> {
  let mut result: Entries = Entries::new();
  let mut duplicate: Entry = Entry::new();
  // Extract swiss settings for variables
  for declaration in declarations(contents)?.lines() {
    // Get identifiers
    let marker: usize = declaration.find(MARKER).unwrap();
    let left: &str = &declaration[..marker].trim_matches(|c: char| "[]".contains(c));
    let identifiers: Vec<&str> = left.split(",").map(|s: &str| s.trim()).collect();
    // Get trail
    let right: &str = &declaration[marker+1..];
    let trail: String = trail(right)?;
    // Save variables and signatures to settings
    for identifier in identifiers {
      if result.contains_key(identifier) {
        duplicate.insert(identifier.to_owned());
      } else {
        result.insert(identifier.to_owned(), if trail.is_empty() { None } else { Some(regex_trim_trail(&trail)?) });
      }
    }
  }
  process_invalid_entries(&format!("duplicate {name}"), &duplicate)?;
  Ok(result)
}

pub fn find_attribute(contents: &str, attribute: &str) -> Res<String> {
  let l_attr: String = format!(r"({ANY}+,{BLANK}*)+");
  let r_attr: String = format!(r"(,{BLANK}*{ANY}+)+");
  let pattern: String = format!(r"^\[({l_attr})?\<&1\>({r_attr})?\]?$");
  editor::edit(&editor::Config::EXTRACT, contents, &pattern, "$0\n", &[attribute], &[])
}

pub fn load(contents: &str) -> Res<Settings> {
  let settings: Settings = Settings {
    vars: load_entries(contents, regex_vars_declarations, regex_vars_trail, "variables' names")?,
  };
  Ok(settings)
}

pub fn process(contents: &str) -> Res<String> {
  let mut haystack: String = contents.to_string();
  // Comment every swiss setting out
  for setting in Settings::ALL {
    let pattern: String = format!(r"^set{BLANK}+&1&2{ANY}*?$");
    haystack = editor::edit(&editor::Config::DEFAULT, &haystack, &pattern, "# $0", &["swiss-", setting], &[])?;
  }
  Ok(haystack)
}
