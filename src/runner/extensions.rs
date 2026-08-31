use gprl::types::Res;

const ANY: &str = r".";
const MARKER: char = '=';
const ID: &str = r"\<.+\>";
const BLANK: &str = r"[[:blank:]]";
const LIST_OP: &str = r"\+\+|\+|/";
const EXPRESSION: &str = r#"\(.*\)|\[.*\]|"[^"]*"|'[^']*'|[^,\s]+"#;

mod optional_arguments {
  use super::*;

  fn regex_declarations(haystack: &str) -> Res<String> {
    let pattern: String = format!(r"^({ID})\((.*{MARKER}.*)\)({BLANK}*:={BLANK}*.*)$");
    let replacement: String = format!("&1{MARKER}&2\n");
    crate::editor::editor(true, haystack, &pattern, &replacement, &[])
  }

  fn regex_value(haystack: &str) -> Res<String> {
    let pattern: String = format!(r"^({EXPRESSION}).*$");
    crate::editor::editor(true, haystack, &pattern, "&1", &[])
  }

  fn regex_fill_out(haystack: &str, name: &str, argc: usize, value: &str) -> Res<String> {
    let sep: &str = if argc == 0 { "" } else { ", " };
    let argv: String = if argc == 0 { String::new() } else {
      // A pattern that matches either a quoted string (which may contain commas) or an unquoted non-comma sequence
      vec![format!(r#"(?:{EXPRESSION})"#); argc].join(r",\s")
    };
    let pattern: String = format!(r"&1\(({argv})\)");
    let replacement: String = format!("{name}(&1{sep}{value})");
    crate::editor::editor(false, haystack, &pattern, &replacement, &[name.to_string()])
  }

  fn regex_truncate(haystack: &str, name: &str, standard: &str) -> Res<String> {
    let pattern: String = format!(r"^\<(&1)\>\(.*\)({BLANK}*:={BLANK}*.*)$");
    let replacement: String = format!("&1({standard})&2");
    crate::editor::editor(false, haystack, &pattern, &replacement, &[name.to_string()])
  }

  pub fn process(content: &str) -> Res<String> {
    let mut haystack: String = content.to_string();
    // Extract declarations for user defined functions from justfile
    for declaration in regex_declarations(&haystack)?.lines() {
      // Get function name and it's extended arguments
      let marker: usize = declaration.find(MARKER).unwrap();
      let name: &str = &declaration[..marker];
      let extended: &str = &declaration[marker+1..];
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
    Ok(haystack)
  }
}

mod runtime_variables {
  use super::*;

  fn regex_declarations(haystack: &str) -> Res<String> {
    let pattern: String = format!(r"^local{BLANK}+({ID}){BLANK}*:={BLANK}*({ANY}*)$");
    let replacement: String = format!("&1 := set('&1', (&2)) && '&1'");
    crate::editor::editor(false, haystack, &pattern, &replacement, &[])
  }

  fn regex_prefixed_assignment(haystack: &str) -> Res<String> {
    let pattern: String = format!(r"^({BLANK}+)({ID}){BLANK}*({LIST_OP}):={BLANK}*({ANY}*)$");
    let replacement: String = format!("&1&2 := var(&2) &3 &4");
    crate::editor::editor(false, haystack, &pattern, &replacement, &[])
  }

  fn regex_assignments(haystack: &str) -> Res<String> {
    let pattern: String = format!(r"^({BLANK}+)({ID}){BLANK}*:={BLANK}*({ANY}*)$");
    let replacement: String = format!("&1{{{{ set(&2, (&3)) }}}}");
    crate::editor::editor(false, haystack, &pattern, &replacement, &[])
  }

  pub fn process(content: &str) -> gprl::types::Res<String> {
    let mut haystack: String = content.to_string();
    haystack = regex_declarations(&haystack)?;
    haystack = regex_prefixed_assignment(&haystack)?;
    haystack = regex_assignments(&haystack)?;
    Ok(haystack)
  }
}

pub fn apply(content: &str) -> gprl::types::Res<String> {
  optional_arguments::process(&runtime_variables::process(content)?)
}
