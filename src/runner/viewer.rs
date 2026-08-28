use gprl::types::Res;

fn list(list: &str) -> String {
  let list: &str = list.trim();
  if list == "" { "there are none" } else { list }.to_string()
}

#[derive(derive_more::Display)]
#[display(r#"# `{version}` {name}

{description}

## Available books

{books}

## Repository

{repository}
"#)]
struct AppViewModel {
  name: String,
  version: String,
  description: String,
  books: String,
  repository: String,
}

pub fn app(books: &str) -> Res::<String> {
  Ok(AppViewModel {
    name: env!("CARGO_PKG_NAME").to_string(),
    version: env!("CARGO_PKG_VERSION").to_string(),
    description: env!("CARGO_PKG_DESCRIPTION").to_string(),
    books: list(books),
    repository: env!("CARGO_PKG_REPOSITORY").to_string(),
  }.to_string())
}

#[derive(derive_more::Display)]
#[display(r#"# {name}

## Available recipes

```console
{recipes}
```
"#)]
struct BookViewModel {
  name: String,
  recipes: String,
}

pub fn book(book: &str, recipes: &str) -> Res::<String> {
  Ok(BookViewModel {
    name: book.to_string(),
    recipes: list(recipes),
  }.to_string())
}
