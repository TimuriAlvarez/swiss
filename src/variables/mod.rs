use gprl::types::Res;

mod fs;

fn runtime_path(signature: &str) -> std::io::Result::<std::path::PathBuf> {
  let xdg: xdg::BaseDirectories = xdg::BaseDirectories::with_prefix(env!("CARGO_PKG_NAME"));
  xdg.create_runtime_directory(signature)
}

pub fn set(signature: &str, name: &str, values: &[String]) -> Res {
  let path: std::path::PathBuf = runtime_path(signature)?.join(name);
  fs::write_to_path(path, values)
}

pub fn next(signature: &str, name: &str, index: Option<usize>) -> Res {
  let path: std::path::PathBuf = runtime_path(signature)?.join(name);
  let index: usize = index.map(|index: usize| index + 1).unwrap_or_default();
  let length: usize = fs::read_length(path).unwrap_or_default();
  if index > length {
    anyhow::bail!("index is out of bounds")
  }
  if index != length {
    println!("{}", index);
  }
  Ok(())
}

pub fn get(signature: &str, name: &str, index: usize) -> Res {
  let path: std::path::PathBuf = runtime_path(signature)?.join(name);
  let value: String = fs::read_value(path, index)?;
  println!("{value}");
  Ok(())
}

pub fn purge(signature: &str) -> Res {
  let path: std::path::PathBuf = runtime_path(signature)?;
  std::fs::remove_dir_all(path)?;
  Ok(())
}
