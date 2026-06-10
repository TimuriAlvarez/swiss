use gprl::types::Res;

#[derive(Debug)]
pub struct VMList {
  pub header: String,
  pub list: Vec::<String>,
  pub placeholder: String,
}

#[derive(Debug)]
pub struct ViewModel {
  pub name: String,
  pub version: String,
  pub description: String,
  pub repository: String,
  pub list: VMList,
}

pub fn presenter(book: &Option<String>) -> Res::<ViewModel> {
  let mut res: ViewModel = ViewModel {
    name: env!("CARGO_PKG_NAME").to_string(),
    version: env!("CARGO_PKG_VERSION").to_string(),
    description: env!("CARGO_PKG_DESCRIPTION").to_string(),
    repository: env!("CARGO_PKG_REPOSITORY").to_string(),
    list: VMList {
      header: "Available recipe books".to_string(),
      list: crate::xdg::books()?,
      placeholder: "It seems like you don't have any recipe books installed".to_string(),
    },
  };
  if let Some(book) = book {
    res.name = book.to_string();
    res.version = format!("evaluate . version");
    res.description = format!("evaluate . description");
    res.repository = format!("evaluate . repository");
    res.list = VMList {
      header: "Available recipes".to_string(),
      list: vec![format!("run . recipes")],
      placeholder: "It seems like this book doesn't contain any recipes".to_string(),
    };
  }
  Ok(res)
}
