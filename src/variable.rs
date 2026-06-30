use gprl::types::Res;

pub fn get(global: bool, path: &String) -> Res {
  let path: std::path::PathBuf = crate::xdg::cache_path(global, None)?.join(path);
  let value: String = std::fs::read_to_string(path).unwrap_or_default();
  print!("{value}");
  Ok(())
}

pub fn set(global: bool, path: &String, value: &String) -> Res {
  let path: std::path::PathBuf = crate::xdg::cache_path(global, None)?.join(path);
  gprl::fs::write_to_path(path, value)?;
  Ok(())
}

pub fn purge(sii: &str) -> Res {
  let path: std::path::PathBuf = crate::xdg::cache_path(false, Some(sii))?;
  std::fs::remove_dir_all(path)?;
  Ok(())
}
