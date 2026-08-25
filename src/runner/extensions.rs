use crate::Res;

mod user_functions {
  use crate::Res;
  const MARKER: char = '=';
  const EXPRESSION: &str = r#""[^"]*"|'[^']*'|[^,]+"#;

  fn regex_declarations(haystack: &str) -> Res::<String> {
    let pattern: String = format!(r"^\<(.*)\>\((.*{MARKER}.*)\)(\s?:=\s?.*)$");
    let replacement: String = format!("&1{MARKER}&2\n");
    crate::editor::editor(true, haystack, &pattern, &replacement, &[])
  }

  fn regex_value(haystack: &str) -> Res::<String> {
    let pattern: String = format!(r"^({EXPRESSION}).*$");
    crate::editor::editor(true, haystack, &pattern, "&1", &[])
  }

  fn regex_fill_out(haystack: &str, name: &str, argc: usize, value: &str) -> Res::<String> {
    let sep: &str = if argc == 0 { "" } else { ", " };
    let argv: String = if argc == 0 { String::new() } else {
      // A pattern that matches either a quoted string (which may contain commas) or an unquoted non-comma sequence
      vec![r#"(?:"[^"]*"|'[^']*'|[^,]+)"#; argc].join(r",\s")
    };
    let pattern: String = format!(r"&1\(({argv})\)");
    let replacement: String = format!("{name}(&1{sep}{value})");
    crate::editor::editor(false, haystack, &pattern, &replacement, &[name.to_string()])
  }

  fn regex_truncate(haystack: &str, name: &str, standard: &str) -> Res::<String> {
    let pattern: String = format!(r"^\<(&1)\>\(.*\)(\s?:=\s?.*)$");
    let replacement: String = format!("&1({standard})&2");
    crate::editor::editor(false, haystack, &pattern, &replacement, &[name.to_string()])
  }

  pub fn process_arguments(content: &str) -> Res::<String> {
    let mut haystack: String = content.to_string();
    // Extract declarations for user defined functions from justfile
    for old in regex_declarations(&haystack)?.lines() {
      // Get function name and it's extended arguments
      let marker: usize = old.find(MARKER).unwrap();
      let name: &str = &old[..marker];
      let extended: &str = &old[marker+1..];
      let mut standard: String = extended.to_string();
      // Iterate over arguments to restore it's standard form and fill omitted values out
      loop {
        if let Some(marker) = standard.find(MARKER) {
          // Split args into parts: left, right and an argument value
          let left: &str = &standard[..marker];
          let trail: &str = &standard[marker+1..];
          let value: String = regex_value(trail)?;
          let right: &str = &trail[value.len()..];
          // Get the current number of optional arguments
          let argc: usize = left.matches(',').count();
          // Reconstruct args without the argument value
          standard = format!("{left}{right}");
          // Fill omitted values out
          haystack = regex_fill_out(&haystack, name, argc, &value)?;
        } else {
          break
        };
      }
      // Make all declarations justfile-compatible
      haystack = regex_truncate(&haystack, name, &standard)?;
    }
    // println!("{haystack}");
    Ok(haystack)
  }
}

pub fn apply(content: &str) -> Res::<String> {
  user_functions::process_arguments(content)
}
