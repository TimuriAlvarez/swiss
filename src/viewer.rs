use gprl::types::Res;
use crate::runner::{spawn, output, Program::*};

#[derive(Debug)]
struct ViewModel {
  pub name: String,
  pub version: String,
  pub description: String,
  pub repository: String,
  pub books: Option::<String>,
  pub recipes: Option::<String>,
}

fn query(temp_file: &temp_file::TempFile, query: &[&str]) -> Res::<String> {
  let query: Vec::<String> = query.into_iter().map(|s| s.to_string()).collect();
  crate::runner::runner(output, Just, &["--justfile"], Some(temp_file), &query)
}

fn presenter(book: &Option::<String>) -> Res::<ViewModel> {
  Ok(if let Some(book) = book {
    let text: String = crate::xdg::book(book)?;
    let text: String = crate::xdg::expand(&text);
    let temp_file: temp_file::TempFile = temp_file::TempFile::with_suffix(".justfile")?.with_contents(&text.into_bytes())?;
    ViewModel {
    name: book.to_string(),
    version: query(&temp_file, &["--evaluate", "version"])?,
    description: query(&temp_file, &["--evaluate", "description"])?,
    repository: query(&temp_file, &["--evaluate", "repository"])?,
    books: None,
    recipes: Some(query(&temp_file, &["--list", "--list-heading", "", "--list-prefix", ""])?),
  }} else { ViewModel {
    name: env!("CARGO_PKG_NAME").to_string(),
    version: env!("CARGO_PKG_VERSION").to_string(),
    description: env!("CARGO_PKG_DESCRIPTION").to_string(),
    repository: env!("CARGO_PKG_REPOSITORY").to_string(),
    books: Some(crate::xdg::books()?),
    recipes: None,
  }})
}

const SYNTAX: &str = "markdown";
const BOOKS_HEADER: &str = "Available books";
const RECIPES_HEADER: &str = "Available recipes";
const BOOKS_PLACEHOLDER: &str = "You don't have any recipe books installed";
const RECIPES_PLACEHOLDER: &str = "It seems that there are no recipes in this book";

impl ViewModel {
  fn markdown(&self) -> String {
    let mut parts: Vec::<String> = Vec::new();
    let mut version: String = self.version.clone();
    if !version.is_empty() {
      version = format!("`{version}` ");
    }
    parts.push(format!("# {version}{}", self.name));
    if !self.description.is_empty() {
      parts.push(self.description.clone());
    }
    if let Some(books) = &self.books {
      parts.push(format!("## {BOOKS_HEADER}"));
      let books: &str = if books.is_empty() { BOOKS_PLACEHOLDER } else { books };
      parts.push(format!("```{SYNTAX}\n{books}\n```"));
    }
    if let Some(recipes) = &self.recipes {
      parts.push(format!("## {RECIPES_HEADER}"));
      let recipes: &str = if recipes.is_empty() { RECIPES_PLACEHOLDER } else { recipes };
      parts.push(format!("```{SYNTAX}\n{recipes}\n```"));
    }
    if !self.repository.is_empty() {
      parts.push(format!("## Repository\n\n<{}>", self.repository));
    }
    parts.join("\n\n")
  }
  pub fn display(&self) -> Res {
    let text: String = self.markdown();
    let temp_file: temp_file::TempFile = temp_file::TempFile::with_suffix(".md")?.with_contents(&text.clone().into_bytes())?;
    if crate::runner::runner(spawn, Glow, &["--width", "0"], Some(&temp_file), &[]).is_ok() { return Ok(()) }
    if crate::runner::runner(spawn, More, &["--silent", "--clean-print"], Some(&temp_file), &[]).is_ok() { return Ok(()) }
    println!("{text}");
    Ok(())
  }
}

pub fn viewer(book: &Option::<String>) -> Res {
  presenter(book)?.display()
}
