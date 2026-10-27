use gprl::types::Res;

#[derive(derive_more::Display)]
#[display(r#"# `{version}` {name}

{description}

## Available books

{books}

## Repository

{repository}
"#)]
struct DefaultMarkdown {
  name: String,
  version: String,
  description: String,
  books: String,
  repository: String,
}

pub fn default(books: &[String]) -> Res<String> {
  Ok(DefaultMarkdown {
    name: env!("CARGO_PKG_NAME").to_string(),
    version: env!("CARGO_PKG_VERSION").to_string(),
    description: env!("CARGO_PKG_DESCRIPTION").to_string(),
    books: if books.is_empty() { format!("there are none") } else { books.join("\n") },
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
struct BookMarkdown {
  name: String,
  recipes: String,
}

pub fn book(book: &str, recipes: &str) -> Res<String> {
  Ok(BookMarkdown {
    name: book.to_string(),
    recipes: recipes.trim_end().to_string(),
  }.to_string())
}
