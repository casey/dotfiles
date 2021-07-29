use crate::common::*;

#[derive(Ord, Eq, PartialEq, PartialOrd, Copy, Clone)]
pub(crate) struct Id {
  number: u32,
}

impl Id {
  pub(crate) fn new(number: u32) -> Self {
    Self { number }
  }

  pub(crate) fn next(self) -> Self {
    Self {
      number: self.number + 1,
    }
  }
}

impl Display for Id {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{:06}", self.number)
  }
}
