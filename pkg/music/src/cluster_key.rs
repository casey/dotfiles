use crate::common::*;

#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub(crate) struct ClusterKey {
  album_artist: String,
  album:        String,
}

impl From<&Import> for ClusterKey {
  fn from(import: &Import) -> Self {
    Self {
      album_artist: import.album_artist().to_owned(),
      album:        import.album().to_owned(),
    }
  }
}

impl Display for ClusterKey {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{}: {}", self.album_artist, self.album)
  }
}
