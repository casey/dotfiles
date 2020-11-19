use crate::common::*;

#[derive(Ord, Eq, PartialEq, PartialOrd, Copy, Clone)]
pub(crate) struct Mp3 {
  id: Id,
}

impl Mp3 {
  pub(crate) fn from_id(id: Id) -> Self {
    Self { id }
  }

  pub(crate) fn file_name(self) -> String {
    format!("{}.mp3", self.id)
  }
}

impl Display for Mp3 {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{}", self.file_name())
  }
}
