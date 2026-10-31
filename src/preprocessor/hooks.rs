use super::prelude::*;

pub fn load(contents: &str, settings: &Settings) -> Res<String> {
  let mut haystack: String = contents.to_owned();
  let mut unbinded: Entry = Entry::new();
  haystack += "\n# swiss-hooks\n";
  for (entry, book) in settings.hook.iter() {
    if let Some(book) = book {
      let book_contents: String = crate::runner::contents(book)?;
      let book_settings: Settings = super::settings::load(&book_contents)?;
      if book_settings.hook.contains_key(entry) {
        haystack += &format!("\n# [private]\n{entry} := __swiss_hook(uuid(), '{book}', '{entry}')\n")
      } else {
        unbinded.insert(entry.to_owned());
      }
    } else {
      // [todo] export hook
    }
  }
  process_invalid_entries("unable to bind hooks", &unbinded)?;
  Ok(haystack)
}

fn regex_assign(haystack: &str, name: &str, value: &str) -> Res<String> {
  let pattern: String = format!(r"^(&{{1}}){BLANK}*:={BLANK}*(.*)$");
  editor::edit(&editor::Config::DEFAULT, haystack, &pattern, "${1} := '&{1}'", &[name], &[value])
}

pub fn process(contents: &str, settings: &Settings, hook: &Hook) -> Res<String> {
  let mut haystack: String = contents.to_owned();
  if !settings.hook.contains_key(&hook.entry) {
    anyhow::bail!("hook invocation forbidden")
  }
  haystack = regex_assign(&haystack, "__swiss_signature", &hook.signature)?;
  haystack = regex_assign(&haystack, "__swiss_hook_variable", &hook.variable)?;
  haystack += &format!("\n[private]\nswiss-hook:\n    {{{{ __swiss_vars_set(false, __swiss_signature, __swiss_hook_variable, {}) }}}}\n", hook.entry);
  Ok(haystack)
}
