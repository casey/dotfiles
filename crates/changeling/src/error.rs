use super::*;

#[derive(Debug, Snafu)]
#[snafu(context(suffix(false)), visibility(pub(crate)))]
pub(crate) enum Error {
  #[snafu(display("failed to run command `{command}`"))]
  CommandInvocation { command: String, source: io::Error },
  #[snafu(display("command `{command}` produced invalid UTF-8"))]
  CommandOutput { command: String, source: Utf8Error },
  #[snafu(display("command `{command}` failed: {stderr}"))]
  CommandStatus { command: String, stderr: String },
  #[snafu(display("failed to read CHANGELOG.md"))]
  ReadChangelog { source: io::Error },
  #[snafu(display("failed to write CHANGELOG.md"))]
  WriteChangelog { source: io::Error },
}
