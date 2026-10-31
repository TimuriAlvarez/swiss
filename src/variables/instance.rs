use gprl::{ipc::Guard, types::Res};

pub struct Instance {
  _guard: Guard,
  signature: String,
  state_path: std::path::PathBuf,
}

impl Instance {
  const PERSIST: bool = false;

  // Convert signature to guard name to avoid name collisions
  fn name(signature: &str) -> String {
    format!("swiss-instance-{signature}")
  }

  pub fn new(signature: &str) -> Res<Self> {
    // Take ownership of the guard so other instances don't purge the state dir for this instance
    let mut guard: Guard = Guard::new(&Self::name(signature))?;
    guard.lock()?;
    // Create the state dir for this instance
    let path: std::path::PathBuf = super::initialize_path(Self::PERSIST, signature)?;
    Ok(Self {
      _guard: guard,
      signature: signature.to_owned(),
      state_path: path.parent().expect("unable to find state directory").to_path_buf(),
    })
  }

  fn stale_state_directories_sweep(&self) -> Res {
    // Collect all signatures from state home path
    let signatures: Vec<String> = crate::utils::dirwalk(&self.state_path, false, |entry: &dirwalk::Entry| {
      entry.is_dir.then(|| entry.relative_path.to_owned())
    })?;
    // Check is instance is orphaned
    for signature in signatures {
      // Acquire instance's guard
      let guard: Guard = Guard::new(&Self::name(&signature))?;
      if guard.is_locked()? { continue }
      // Purge orphaned instance
      super::purge(Self::PERSIST, &signature)?;
    }
    Ok(())
  }
}

impl Drop for Instance {
  fn drop(&mut self) {
    // Purge own state directory
    super::purge(Self::PERSIST, &self.signature).expect("state purge failed");
    // Purge all stale state directories
    self.stale_state_directories_sweep().expect("stale state directories sweep failed");
  }
}
