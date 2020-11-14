use crate::common::*;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub(crate) enum Format {
  Flac,
  Mp3,
}

impl TryFrom<&Path> for Format {
  type Error = Error;

  fn try_from(path: &Path) -> Result<Self, Self::Error> {
    let extension = path.extension().unwrap_or_default();

    if extension == "mp3" {
      Ok(Self::Mp3)
    } else if extension == "flac" {
      Ok(Self::Flac)
    } else {
      Err(anyhow!("Unknown format: `{}`", path.display()))
    }
  }
}
