use super::*;

#[derive(Debug, Snafu)]
#[snafu(context(suffix(false)), visibility(pub(crate)))]
pub(crate) enum Error {
  #[snafu(display("failed to create directory `{path}`"))]
  CreateDir {
    backtrace: Option<Backtrace>,
    path: Utf8PathBuf,
    source: io::Error,
  },
  #[snafu(display("rsync failed for `{path}`: {status}"))]
  Download {
    backtrace: Option<Backtrace>,
    path: String,
    status: ExitStatus,
  },
  #[snafu(display("failed to write files-from list for `{path}`"))]
  FilesFrom {
    backtrace: Option<Backtrace>,
    path: String,
    source: io::Error,
  },
  #[snafu(display("failed to run `rsync --list-only`"))]
  Listing {
    backtrace: Option<Backtrace>,
    source: io::Error,
  },
  #[snafu(display("failed to parse rsync listing line `{line}`"))]
  ListingParse {
    backtrace: Option<Backtrace>,
    line: String,
  },
  #[snafu(display("`rsync --list-only` failed: {status}"))]
  ListingStatus {
    backtrace: Option<Backtrace>,
    status: ExitStatus,
  },
  #[snafu(display("failed to read rsync progress for `{path}`"))]
  ProgressRead {
    backtrace: Option<Backtrace>,
    path: String,
    source: io::Error,
  },
  #[snafu(display("failed to spawn rsync for `{path}`"))]
  Spawn {
    backtrace: Option<Backtrace>,
    path: String,
    source: io::Error,
  },
  #[snafu(display("failed to build progress style"))]
  Style {
    backtrace: Option<Backtrace>,
    source: TemplateError,
  },
  #[snafu(display("worker thread panicked"))]
  Thread { backtrace: Option<Backtrace> },
  #[snafu(display("failed to wait for rsync for `{path}`"))]
  Wait {
    backtrace: Option<Backtrace>,
    path: String,
    source: io::Error,
  },
}
