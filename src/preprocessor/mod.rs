use gprl::types::Res;

pub mod prelude;
mod settings;
mod arguments;
mod regions;
mod variables;

pub fn has_attribute(contents: &str, attribute: &str) -> Res<bool> {
  settings::find_attribute(contents, attribute).map(|s: String| !s.is_empty())
}

pub fn run(contents: &str) -> Res<String> {
  // Source default justfile and active settings
  let swiss: &str = include_str!("../../resources/swiss.just");
  let mut contents: String = format!("{contents}\n{swiss}");
  let settings: prelude::Settings = settings::load(&contents)?;
  // Process macro expansions
  contents = settings::process(&contents)?;
  contents = arguments::process(&contents)?;
  contents = regions::process(&contents)?;
  contents = variables::process(&contents, &settings)?;
  Ok(contents)
}
