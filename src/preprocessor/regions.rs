use super::prelude::*;

#[derive(Debug)]
struct Line {
  pub text: String,
  pub indent: String,
}

const REPLACE_ONCE: usize = 1;
const REGION_BEGIN_LEN: usize = 4;
const REGION_END_LEN: usize = 3;
const UNICODE_PRIVATE_USE_AREA: std::ops::RangeInclusive<char> = '\u{E000}'..='\u{F8FF}';

fn select_separator_for(haystack: &str) -> Res<char> {
  for c in UNICODE_PRIVATE_USE_AREA {
    if !haystack.contains(c) { return Ok(c) }
  }
  anyhow::bail!("unable to pick a separator")
}

fn regex_regions(haystack: &str, separator: char) -> Res<String> {
  let pattern: String = format!(r#"{BLANK}+(@{BLANK}*)?{REGION_BEGIN}({ANY}|\n)*?{REGION_END}{ANY}*"#);
  let replacement: String = format!("$0{separator}");
  crate::editor::editor(true, haystack, &pattern, &replacement, &[])
}

fn initial_line(text: &str) -> String {
  if text.is_empty() { return "[]".to_owned() }
  format!("# {text}")
}

fn count_of(text: &str, charset: &str) -> usize {
  text.chars().filter(|c: &char| charset.contains(*c)).count()
}

pub fn process(contents: &str) -> Res<String> {
  let mut haystack: String = contents.to_string();
  let separator: char = select_separator_for(&haystack)?;
  // Extract regions from justfile
  for region in regex_regions(&haystack, separator)?.split(separator) {
    if region.is_empty() { continue }
    // Extract echo flag
    let mut lines: Vec<Line> = Vec::new();
    // Split each line into text and indent
    for line in region.lines() {
      let indent: String = line.chars().take_while(|c: &char| *c == ' ' || *c == '\t').collect();
      lines.push(Line {
        text: line[indent.len()..].to_owned(),
        indent: indent,
      });
    }
    // Trim the first line
    let single_case: bool = lines.len() == 1;
    let first: &mut Line = lines.first_mut().expect("invalid region format");
    let quiet: &str = if first.text.starts_with('@') { "@ " } else { "" };
    first.text = first.text.trim_start_matches(|c: char| "@ \t".contains(c))[REGION_BEGIN_LEN..].trim_start().to_owned();
    if !single_case { first.text = initial_line(&first.text); }
    // Trim the last line
    let last: &mut Line = lines.last_mut().expect("invalid region format");
    last.text = last.text[REGION_END_LEN..].trim_start().to_owned();
    last.text = initial_line(&last.text);
    // Aggregate result
    let mut balance: usize = 0;
    let result: Vec<String> = lines.into_iter().map(|line: Line|
      if line.text.starts_with('#') { format!("{}{quiet}{}", line.indent, line.text) }
      else {
        let echo: &str = if balance == 0 { quiet } else { "" };
        let begin: &str = if balance == 0 { "{{ " } else { "" };
        balance += count_of(&line.text, "{([") - count_of(&line.text, "])}");
        let end: &str = if balance == 0 { " }}" } else { "" };
        format!("{}{echo}{begin}{}{end}", line.indent, line.text)
      }
    ).collect();
    // Replace the region with result
    haystack = haystack.replacen(region, &result.join("\n"), REPLACE_ONCE);
  }
  Ok(haystack)
}
