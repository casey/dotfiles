use super::*;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Repo {
  pub(crate) owner: String,
  pub(crate) name: String,
}

impl Display for Repo {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{}/{}", self.owner, self.name)
  }
}

impl FromStr for Repo {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self> {
    if let Some((owner, name)) = s.split_once('/')
      && !owner.is_empty()
      && !name.is_empty()
      && !name.contains('/')
    {
      Ok(Self {
        owner: owner.into(),
        name: name.into(),
      })
    } else {
      error::RepoParse { repo: s }.fail()
    }
  }
}
