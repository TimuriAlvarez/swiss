use gprl::types::Res;

mod fs;

pub fn initialize_path(persist: bool, signature: &str) -> std::io::Result::<std::path::PathBuf> {
  let xdg: xdg::BaseDirectories = xdg::BaseDirectories::with_prefix(env!("CARGO_PKG_NAME"));
  if persist { xdg.create_config_directory(signature) } else { xdg.create_state_directory(signature) }
}

pub fn set(persist: bool, signature: &str, name: &str, values: &[String]) -> Res {
  let path: std::path::PathBuf = initialize_path(persist, signature)?.join(name);
  fs::write_to_path(path, values)
}

pub fn next(persist: bool, signature: &str, name: &str, index: Option<usize>) -> Res {
  let path: std::path::PathBuf = initialize_path(persist, signature)?.join(name);
  let index: usize = index.map(|index: usize| index + 1).unwrap_or_default();
  let length: usize = fs::read_length(path).unwrap_or_default();
  if index > length {
    anyhow::bail!("index is out of bounds")
  }
  if index != length {
    println!("{index}");
  }
  Ok(())
}

pub fn get(persist: bool, signature: &str, name: &str, index: usize) -> Res {
  let path: std::path::PathBuf = initialize_path(persist, signature)?.join(name);
  let value: String = fs::read_value(path, index)?;
  println!("{value}");
  Ok(())
}

pub fn purge(persist: bool, signature: &str) -> Res {
  let path: std::path::PathBuf = initialize_path(persist, signature)?;
  std::fs::remove_dir_all(path)?;
  Ok(())
}
