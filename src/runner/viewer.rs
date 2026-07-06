#[derive(derive_more::Display)]
#[display(r#"# `{version}` {name}

{description}

## Repository

{repository}
"#)]
pub struct AppViewModel {
  name: String,
  version: String,
  description: String,
  repository: String,
}

impl AppViewModel {
  pub fn new() -> Self {
    Self {
      name: env!("CARGO_PKG_NAME").to_string(),
      version: env!("CARGO_PKG_VERSION").to_string(),
      description: env!("CARGO_PKG_DESCRIPTION").to_string(),
      repository: env!("CARGO_PKG_REPOSITORY").to_string(),
    }
  }
}

#[derive(derive_more::Display)]
#[display(r#"# {name}

## Available recipes

```console
{list}
```
"#)]
pub struct BookViewModel {
  name: String,
  list: String,
}

impl BookViewModel {
  pub fn new(book: &str, recipes: &str) -> Self {
    let recipes: &str = recipes.trim();
    let recipes: &str = if recipes == "" { "there are none" } else { recipes };
    Self {
      name: book.to_string(),
      list: recipes.to_string(),
    }
  }
}
