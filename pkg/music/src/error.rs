use crate::common::*;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub(crate) enum Error {
  Filesystem {
    path:   PathBuf,
    source: io::Error,
  },
  Walkdir {
    source: walkdir::Error,
  },
  DuplicateTranscodes {
    a: PathBuf,
    b: PathBuf,
  },
  Id3 {
    source: id3::Error,
    path:   PathBuf,
  },
  Flac {
    source: metaflac::Error,
    path:   PathBuf,
  },
  VorbisCommentMissing {
    path: PathBuf,
  },
  HomeDir,
  Timestamp {
    source: ParseIntError,
    path:   PathBuf,
    text:   String,
  },
  Comments {
    path:     PathBuf,
    comments: Vec<String>,
  },
  DiscNumber {
    path:   PathBuf,
    number: Option<u32>,
  },
  TrackNumber {
    path:   PathBuf,
    number: Option<u32>,
  },
  TrackAlbum {
    path: PathBuf,
  },
  TrackYear {
    path: PathBuf,
  },
  TrackAlbumArtist {
    path: PathBuf,
  },
  TrackName {
    path: PathBuf,
  },
  Rename {
    src:    PathBuf,
    dst:    PathBuf,
    source: io::Error,
  },
  RenameDstExists {
    path: PathBuf,
  },
  TranscodeInvoke {
    source: io::Error,
  },
  TranscodeStatus {
    status: ExitStatus,
    stderr: String,
  },
}

impl From<walkdir::Error> for Error {
  fn from(source: walkdir::Error) -> Self {
    Error::Walkdir { source }
  }
}
