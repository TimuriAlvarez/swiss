pub struct Instance {
  signature: String,
}

impl Instance {
  const PERSIST: bool = false;

  pub fn new(signature: &str) -> gprl::types::Res<Self> {
    super::initialize_path(Self::PERSIST, signature)?;
    Ok(Self {
      signature: signature.to_owned(),
    })
  }
}

impl Drop for Instance {
  fn drop(&mut self) {
    let _ = super::purge(Self::PERSIST, &self.signature);
  }
}
