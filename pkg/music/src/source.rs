use crate::common::*;

pub(crate) enum Source {
  Mp3(PathBuf),
  Flac(PathBuf),
  Both { mp3: PathBuf, flac: PathBuf },
}

impl Source {
  pub(crate) fn mp3(path: PathBuf) -> Source {
    assert_eq!(path.extension(), Some(OsStr::new("mp3")));
    Source::Mp3(path)
  }

  pub(crate) fn flac(path: PathBuf) -> Source {
    assert_eq!(path.extension(), Some(OsStr::new("flac")));
    Source::Flac(path)
  }

  pub(crate) fn both(flac: PathBuf, mp3: PathBuf) -> Source {
    assert_eq!(mp3.extension(), Some(OsStr::new("mp3")));
    assert_eq!(flac.extension(), Some(OsStr::new("flac")));
    Source::Both { flac, mp3 }
  }
}
