use super::prelude::*;

fn regex_var_assignments(haystack: &str) -> Res<String> {
  let pattern: String = format!(r#"({IDENT}){BLANK}*({OPERATOR})?:={BLANK}*\{{{BLANK}*({ANY}{ANY}*?){BLANK}*}}"#);
  let replacement: String = format!("$1\n");
  editor::edit(&editor::Config::EXTRACT, haystack, &pattern, &replacement, &[], &[])
}

fn unwrap_signature(signature: &Option<String>) -> String {
  if let Some(signature) = signature {
    format!("'{signature}'")
  } else {
    "__swiss_signature".to_owned()
  }
}

fn regex_fill_assignments_out(haystack: &str, name: &str, signature: &Option<String>) -> Res<String> {
  let persist: bool = signature.is_some();
  let signature: String = unwrap_signature(signature);
  let pattern: String = format!(r#"(&1){BLANK}*:={BLANK}*\{{{BLANK}*({ANY}{ANY}*?){BLANK}*}}"#);
  let replacement: String = format!("__swiss_vars_set({persist}, {signature}, '{name}', $2)");
  editor::edit(&editor::Config::DEFAULT, haystack, &pattern, &replacement, &[name], &[])
}

fn regex_fill_assignments_with_operators_out(haystack: &str, name: &str, signature: &Option<String>) -> Res<String> {
  let persist: bool = signature.is_some();
  let signature: String = unwrap_signature(signature);
  let pattern: String = format!(r#"(&1){BLANK}*({OPERATOR}):={BLANK}*\{{{BLANK}*({ANY}{ANY}*?){BLANK}*}}"#);
  let replacement: String = format!("__swiss_vars_set({persist}, {signature}, '{name}', var({name}) $2 $3)");
  editor::edit(&editor::Config::DEFAULT, haystack, &pattern, &replacement, &[name], &[])
}

fn regex_var_calls(haystack: &str) -> Res<String> {
  let pattern: String = format!(r#"([^{CHAR}])var\(({IDENT})\)"#);
  let replacement: String = format!("$2\n");
  editor::edit(&editor::Config::EXTRACT, haystack, &pattern, &replacement, &[], &[])
}

fn regex_fill_calls_out(haystack: &str, name: &str, signature: &Option<String>) -> Res<String> {
  let persist: bool = signature.is_some();
  let signature: String = unwrap_signature(signature);
  let pattern: String = format!(r#"([^{CHAR}])var\((&1)\)"#);
  let replacement: String = format!("${{1}}__swiss_vars_get({persist}, {signature}, '{name}', [])");
  editor::edit(&editor::Config::DEFAULT, haystack, &pattern, &replacement, &[name], &[])
}

pub fn process(contents: &str, settings: &Settings) -> Res<String> {
  let mut haystack: String = contents.to_string();
  let mut unresolved: Entry = Entry::new();
  // Extract assigned variables' names
  let names: Entry = regex_var_assignments(&haystack)?.lines().map(|line: &str| line.to_owned()).collect();
  // Substitute variable's assignments
  for name in names {
    if let Some(signature) = settings.vars.get(&name) {
      haystack = regex_fill_assignments_out(&haystack, &name, signature)?;
      haystack = regex_fill_assignments_with_operators_out(&haystack, &name, signature)?;
    } else {
      unresolved.insert(name);
    }
  }
  // Extract accessed variables' names
  let names: Entry = regex_var_calls(&haystack)?.lines().map(|line: &str| line.to_owned()).collect();
  // Substitute variable's access calls
  for name in names {
    if let Some(signature) = settings.vars.get(&name) {
      haystack = regex_fill_calls_out(&haystack, &name, signature)?;
    } else {
      unresolved.insert(name);
    }
  }
  // Report unresolved variables' names
  process_invalid_entries("unresolved variables' names", &unresolved)?;
  Ok(haystack)
}
