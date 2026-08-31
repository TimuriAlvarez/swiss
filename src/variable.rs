use gprl::types::Res;

fn runtime_path(signature: &str) -> std::io::Result::<std::path::PathBuf> {
  let xdg: xdg::BaseDirectories = xdg::BaseDirectories::with_prefix(env!("CARGO_PKG_NAME"));
  xdg.create_runtime_directory(signature)
}

pub fn set(signature: &str, name: &str, values: &[String]) -> Res {
  let len: usize = values.len();
  let path: std::path::PathBuf = runtime_path(signature)?;
  for index in 0..len {
    gprl::fs::write_to_path(path.join(format!("{name}.part{index}")), &values[index])?;
  }
  gprl::fs::write_to_path(path.join(format!("{name}.data")), format!("{len}"))?;
  Ok(())
}

pub fn next(signature: &str, name: &str, index: Option<usize>) -> Res {
  let path: std::path::PathBuf = runtime_path(signature)?;
  let len: String = std::fs::read_to_string(path.join(format!("{name}.data")))?;
  let len: usize = len.parse::<usize>()?;
  let index: usize = index.map(|index: usize| index + 1).unwrap_or_default();
  if index < len {
    println!("{}", index);
  }
  Ok(())
}

pub fn get(signature: &str, name: &str, index: usize) -> Res {
  let path: std::path::PathBuf = runtime_path(signature)?;
  let value: String = std::fs::read_to_string(path.join(format!("{name}.part{index}")))?;
  println!("{value}");
  Ok(())
}

pub fn purge(signature: &str) -> Res {
  let path: std::path::PathBuf = runtime_path(signature)?;
  std::fs::remove_dir_all(path)?;
  Ok(())
}
