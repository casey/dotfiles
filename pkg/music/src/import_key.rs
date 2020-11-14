use crate::common::*;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub(crate) struct ImportKey {
  disc_number:  u32,
  track_number: u32,
}

impl ImportKey {
  pub(crate) fn disc_number(self) -> u32 {
    self.disc_number
  }

  pub(crate) fn track_number(self) -> u32 {
    self.track_number
  }
}

impl From<&Import> for ImportKey {
  fn from(import: &Import) -> Self {
    Self {
      disc_number:  import.disc_number(),
      track_number: import.track_number(),
    }
  }
}
