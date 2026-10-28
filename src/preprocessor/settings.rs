use super::prelude::*;

fn regex_declarations(haystack: &str) -> Res<String> {
  let pattern: String = format!(r#"^$1{BLANK}+$2{BLANK}+:={BLANK}+({EXPRESSION})(({BLANK}+$3{BLANK}+$4{BLANK}+('{IDENT}'|"{IDENT}"))?)$"#);
  let replacement: String = format!("$1{MARKER}$2\n");
  crate::editor::editor(true, haystack, &pattern, &replacement, &["set", "swiss-vars", "persist", "as"])
}

fn regex_signature(haystack: &str) -> Res<String> {
  let pattern: String = format!(r#"^{BLANK}+$3{BLANK}+$4{BLANK}+('{IDENT}'|"{IDENT}")$"#);
  crate::editor::editor(true, haystack, &pattern, "$1", &["set", "swiss-vars", "persist", "as"])
}

fn load_vars(contents: &str) -> Res<Variables> {
  let mut result: Variables = Variables::new();
  let mut duplicate: VarNames = VarNames::new();
  // Extract swiss settings for variables
  for declaration in regex_declarations(contents)?.lines() {
    // Get variables' names
    let marker: usize = declaration.find(MARKER).unwrap();
    let left: &str = &declaration[..marker].trim_matches(|c: char| "[]".contains(c));
    let names: Vec<&str> = left.split(",").map(|s: &str| s.trim()).collect();
    // Get signature (for persistent variables)
    let trail: &str = &declaration[marker+1..];
    let signature: String = regex_signature(trail)?;
    // Save variables and signatures to settings
    for name in names {
      if result.contains_key(name) {
        duplicate.insert(name.to_owned());
      } else {
        result.insert(name.to_owned(), if signature.is_empty() { None } else { Some(signature.clone()) });
      }
    }
  }
  process_invalid_names("duplicate variables' names", &duplicate)?;
  Ok(result)
}

pub fn find_attribute(contents: &str, attribute: &str) -> Res<String> {
  let l_attr: String = format!(r"({ANY}+,{BLANK}*)+");
  let r_attr: String = format!(r"(,{BLANK}*{ANY}+)+");
  let pattern: String = format!(r"^\[({l_attr})?\<$1\>({r_attr})?\]?$");
  let replacement: String = format!("$0\n");
  crate::editor::editor(true, contents, &pattern, &replacement, &[attribute])
}

pub fn load(contents: &str) -> Res<Settings> {
  let settings: Settings = Settings {
    vars: load_vars(contents)?,
  };
  Ok(settings)
}

pub fn process(contents: &str) -> Res<String> {
  let mut haystack: String = contents.to_string();
  // Comment every swiss setting out
  for setting in Settings::ALL {
    let pattern: String = format!(r"^set{BLANK}+$1$2{ANY}*?$");
    let replacement: String = format!("# $0");
    haystack = crate::editor::editor(false, &haystack, &pattern, &replacement, &["swiss-", setting])?;
  }
  Ok(haystack)
}
