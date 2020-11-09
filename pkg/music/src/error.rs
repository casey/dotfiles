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
  Rename {
    src:    PathBuf,
    dst:    PathBuf,
    source: io::Error,
  },
}
