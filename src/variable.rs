use crate::Res;

fn cache_path(global: bool, signature: Option::<&str>) -> std::io::Result::<std::path::PathBuf> {
  let xdg: xdg::BaseDirectories = xdg::BaseDirectories::with_prefix(env!("CARGO_PKG_NAME"));
  let path: &str = if global {
    "variables"
  } else if let Some(signature) = signature {
    signature
  } else {
    &std::env::var("SWISS_SIGNATURE").expect("environment variable SWISS_SIGNATURE")
  };
  xdg.create_cache_directory(path)
}

pub fn get(global: bool, path: &String) -> Res {
  let path: std::path::PathBuf = cache_path(global, None)?.join(path);
  let value: String = std::fs::read_to_string(path).unwrap_or_default();
  print!("{value}");
  Ok(())
}

pub fn set(global: bool, path: &String, value: &String) -> Res {
  let path: std::path::PathBuf = cache_path(global, None)?.join(path);
  gprl::fs::write_to_path(path, value)?;
  Ok(())
}

pub fn purge(signature: &str) -> Res {
  let path: std::path::PathBuf = cache_path(false, Some(signature))?;
  std::fs::remove_dir_all(path)?;
  Ok(())
}
