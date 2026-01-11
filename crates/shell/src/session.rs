use crate::common::*;

#[derive(Deserialize, Debug)]
pub struct Session {
  pub name:     String,
  pub attached: u32,
}

impl Session {
  pub fn is_detached(&self) -> bool {
    self.attached == 0
  }

  pub fn name(&self) -> &str {
    &self.name
  }
}
