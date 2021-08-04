use crate::common::*;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
  #[snafu(display("Could not deserialize config `{}`: {}", path.display(), source))]
  ConfigDeserialize {
    path:   PathBuf,
    source: serde_yaml::Error,
  },
  #[snafu(display("Could not get current directory: {}", source))]
  CurrentDir { source: io::Error },
  #[snafu(display("I/O error at `{}`: {}", path.display(), source))]
  FilesystemIo { path: PathBuf, source: io::Error },
  #[snafu(display("Failed to get home directory."))]
  HomeDir,
  #[snafu(display("Could not find project root."))]
  NotFound,
  #[snafu(display("Path was not valid unicode: {}", path.display()))]
  PathDecode { path: PathBuf },
}
