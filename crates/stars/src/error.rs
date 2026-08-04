use super::*;

#[derive(Debug, Snafu)]
#[snafu(context(suffix(false)), visibility(pub(crate)))]
pub(crate) enum Error {
  #[snafu(display("failed to deserialize cache at `{path}`"))]
  CacheDeserialize {
    backtrace: Option<Backtrace>,
    path: Utf8PathBuf,
    source: serde_json::Error,
  },
  #[snafu(display("failed to locate user cache directory"))]
  CacheDir { backtrace: Option<Backtrace> },
  #[snafu(display("failed to read cache at `{path}`"))]
  CacheRead {
    backtrace: Option<Backtrace>,
    path: Utf8PathBuf,
    source: io::Error,
  },
  #[snafu(display("failed to write cache at `{path}`"))]
  CacheWrite {
    backtrace: Option<Backtrace>,
    path: Utf8PathBuf,
    source: io::Error,
  },
  #[snafu(display("failed to create directory `{path}`"))]
  CreateDir {
    backtrace: Option<Backtrace>,
    path: Utf8PathBuf,
    source: io::Error,
  },
  #[snafu(display("HTTP request failed"))]
  Http {
    backtrace: Option<Backtrace>,
    source: ureq::Error,
  },
  #[snafu(display("failed to write chart to `{path}`"))]
  HtmlWrite {
    backtrace: Option<Backtrace>,
    path: Utf8PathBuf,
    source: io::Error,
  },
  #[snafu(display("failed to open `{path}` in browser"))]
  Open {
    backtrace: Option<Backtrace>,
    path: Utf8PathBuf,
    source: io::Error,
  },
  #[snafu(display("cache directory `{}` is not unicode", path.display()))]
  PathUnicode {
    backtrace: Option<Backtrace>,
    path: PathBuf,
  },
  #[snafu(display("repository `{repo}` not found"))]
  RepoNotFound {
    backtrace: Option<Backtrace>,
    repo: Repo,
  },
  #[snafu(display("invalid repository `{repo}`, expected `OWNER/REPO`"))]
  RepoParse {
    backtrace: Option<Backtrace>,
    repo: String,
  },
  #[snafu(display("failed to serialize JSON"))]
  Serialize {
    backtrace: Option<Backtrace>,
    source: serde_json::Error,
  },
  #[snafu(display("GitHub API returned status {status}"))]
  Status {
    backtrace: Option<Backtrace>,
    status: u16,
  },
  #[snafu(display("failed to build progress style"))]
  Style {
    backtrace: Option<Backtrace>,
    source: TemplateError,
  },
  #[snafu(display("failed to run `gh auth token`"))]
  Token {
    backtrace: Option<Backtrace>,
    source: io::Error,
  },
  #[snafu(display("GitHub token is empty"))]
  TokenEmpty { backtrace: Option<Backtrace> },
  #[snafu(display("`gh auth token` failed: {status}"))]
  TokenStatus {
    backtrace: Option<Backtrace>,
    status: ExitStatus,
  },
}
