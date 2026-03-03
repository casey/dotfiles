use crate::common::*;

#[derive(Debug)]
pub(crate) enum Error {
  #[allow(dead_code)]
  Io {
    io_error: io::Error,
  },
  #[allow(dead_code)]
  Deserialize {
    deserialize_error: serde_json::Error,
  },
  #[allow(dead_code)]
  ExitStatus {
    exit_status: ExitStatus,
  },
  #[allow(dead_code)]
  Launch {
    binary: String,
  },
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl From<io::Error> for Error {
  fn from(io_error: io::Error) -> Error {
    Error::Io { io_error }
  }
}

impl From<serde_json::Error> for Error {
  fn from(deserialize_error: serde_json::Error) -> Error {
    Error::Deserialize { deserialize_error }
  }
}
