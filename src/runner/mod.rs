use crate::Res;

mod viewer;
mod shell;

pub fn viewer(book: &Option::<String>) -> Res {
  let markdown: String = if let Some(book) = book {
    let recipes: shell::Output = shell::run(shell::output, shell::JUST, &["--justfile", book, "--list", "--list-heading", "", "--list-prefix", "", "--color", "always"], None, &[])?;
    if !recipes.success {
      anyhow::bail!("unable to retrieve recipes from `{book}` book\n\n{}", recipes.stderr)
    }
    viewer::BookViewModel::new(book, &recipes.stdout).to_string()
  } else {
    viewer::AppViewModel::new().to_string()
  };
  let temp_file: temp_file::TempFile = temp_file::with_contents(&markdown.into_bytes());
  shell::run(shell::spawn, shell::GLOW, &["--width", "0"], Some(&temp_file), &[]).map(|_| ())
}

pub fn runner(book: &str, args: &[String]) -> Res {
  shell::run(shell::spawn, shell::JUST, &["--justfile", book], None, args).map(|_| ())
}
