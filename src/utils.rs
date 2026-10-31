use gprl::types::Res;
use lexical_sort::StringSort;

pub fn dirwalk(path: &std::path::PathBuf, sort: bool, filter: impl Fn(&dirwalk::Entry) -> Option<String>) -> Res<Vec<String>> {
  let mut result: Vec<String> = Vec::new();
  // Walk through directory
  let entries: Vec<dirwalk::Entry> = dirwalk::WalkBuilder::new(path).build()?.entries;
  // Filter entries
  entries.into_iter().for_each(|entry: dirwalk::Entry| {
    if let Some(entry) = filter(&entry) {
      result.push(entry);
    }
  });
  // Sort the result
  if sort {
    result.string_sort(lexical_sort::natural_lexical_cmp);
  }
  Ok(result)
}
