use crate::common::*;

#[derive(Ord, Eq, PartialEq, PartialOrd, Copy, Clone)]
pub(crate) struct Flac {
  id: Id,
}

impl Flac {
  pub(crate) fn from_id(id: Id) -> Self {
    Self { id }
  }

  pub(crate) fn id(self) -> Id {
    self.id
  }

  pub(crate) fn file_name(self) -> String {
    format!("{}.flac", self.id)
  }
}
