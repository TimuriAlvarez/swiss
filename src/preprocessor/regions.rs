use super::prelude::*;

fn lines_from_region(region: &str) -> RegionLines {
  let mut rl: RegionLines = RegionLines::new();
  if region.is_empty() { return rl }
  for line in region.lines() {
    let indent: String = line.chars().take_while(|c: &char| *c == ' ' || *c == '\t').collect();
    rl.lines.push(RegionLine {
      text: line[indent.len()..].to_owned(),
      indent: indent,
      is_border: false,
      is_rightmost: false,
    });
  }
  rl
}

fn select_separator_for(haystack: &str) -> Res<char> {
  const UNICODE_PRIVATE_USE_AREA: std::ops::RangeInclusive<char> = '\u{E000}'..='\u{F8FF}';
  for c in UNICODE_PRIVATE_USE_AREA {
    if !haystack.contains(c) { return Ok(c) }
  }
  anyhow::bail!("unable to pick a separator")
}

fn regex_regions(haystack: &str, region_begin: &str, region_end: &str, separator: char) -> Res<String> {
  let pattern: String = format!(r#"{BLANK}+(@{BLANK}*)?{region_begin}({ANY}|\n)*?{region_end}{ANY}*"#);
  let replacement: String = format!("$0{separator}");
  editor::edit(&editor::Config::EXTRACT, haystack, &pattern, &replacement, &[], &[])
}

fn regex_keyword_end(haystack: &str) -> Res<String> {
  let pattern: String = format!(r#"^{ANY}*?\<(&1|&2)\>$"#);
  editor::edit(&editor::Config::EXTRACT, haystack, &pattern, "$1", &["then", "else"], &[])
}

fn count_of(s: &str, charset: &str) -> usize {
  s.chars().filter(|c: &char| charset.contains(*c)).count()
}

fn real_length(region: &str) -> usize {
  region.len() - count_of(region, r"\")
}

fn border_line(text: &str, default_border: &str) -> String {
  if text.is_empty() { return default_border.to_owned() }
  format!("# {text}")
}

fn process_regions(haystack: &str, region_begin: &str, region_end: &str, separator: char, default_border: &str, canonize: impl Fn(&RegionLines) -> Vec<String>) -> Res<String> {
  let mut haystack: String = haystack.to_string();
  // Extract script regions from justfile
  for region in regex_regions(&haystack, region_begin, region_end, separator)?.split(separator) {
    // Split region into lines
    let mut rl: RegionLines = lines_from_region(region);
    let single_case: bool = rl.lines.len() == 1;
    // Trim the first line
    if let Some(first) = rl.lines.first_mut() {
      rl.echo = first.text.starts_with('@');
      first.text = first.text.trim_start_matches(|c: char| "@ \t".contains(c))[real_length(region_begin)..].trim_start().to_owned();
      if !single_case {
        first.text = border_line(&first.text, default_border);
        first.is_border = true;
      }
    }
    // Trim the last line
    if let Some(last) = rl.lines.last_mut() {
      last.text = last.text[real_length(region_end)..].trim_start().to_owned();
      last.text = border_line(&last.text, default_border);
      last.is_border = true;
    }
    // Find the rightmost line and mark it
    rl.lines.iter_mut().rev().find(|line: &&mut RegionLine| !line.is_border).map(|line: &mut RegionLine| line.is_rightmost = true);
    // Canonize all lines
    let result: Vec<String> = canonize(&rl);
    // Replace the region with result
    const COUNT_ONCE: usize = 1;
    haystack = haystack.replacen(region, &result.join("\n"), COUNT_ONCE);
  }
  Ok(haystack)
}

pub fn process(contents: &str) -> Res<String> {
  let mut haystack: String = contents.to_string();
  // Find a separator that does not occur in haystack
  let separator: char = select_separator_for(&haystack)?;
  // Canonize interpolation blocks
  const DEFAULT_BORDER_INTERPOL: &str = "[]";
  haystack = process_regions(&haystack, REGION_BEGIN_INTERPOL, REGION_END_INTERPOL, separator, DEFAULT_BORDER_INTERPOL, |rl: &RegionLines| {
    let mut balance: usize = 0;
    rl.lines.iter().map(|line: &RegionLine| {
      if line.text.starts_with('#') { format!("{}{}{}", line.indent, rl.echo(true), line.text) }
      else {
        let echo: &str = rl.echo(balance == 0);
        let begin: &str = if balance == 0 { "{{ " } else { "" };
        balance += count_of(&line.text, "{([") - count_of(&line.text, "])}");
        let end: &str = if balance == 0 { " }}" } else { "" };
        format!("{}{echo}{begin}{}{end}", line.indent, line.text)
      }
    }).collect()
  })?;
  // Canonize script blocks
  const DEFAULT_BORDER_SCRIPT: &str = "{{ [] }}";
  haystack = process_regions(&haystack, REGION_BEGIN_SCRIPT, REGION_END_SCRIPT, separator, DEFAULT_BORDER_SCRIPT, |rl: &RegionLines| {
    let mut is_first: bool = true;
    rl.lines.iter().map(|line: &RegionLine| {
      let echo: &str = rl.echo(line.is_border || is_first);
      if line.is_border { format!("{}{echo}{}", line.indent, line.text) }
      else if line.text.starts_with('#') { format!(r"{}\", line.indent) }
      else {
        is_first = false;
        let terminator: &str = if line.is_rightmost { "" }
        else if regex_keyword_end(&line.text).expect("regex failed").is_empty() { r"; \" }
        else { r" \" };
        format!("{}{echo}{}{terminator}", line.indent, line.text)
      }
    }).collect()
  })?;
  Ok(haystack)
}
