use crate::common::*;

#[derive(Debug)]
pub enum Error {
  Status { status: i32, stderr: String },
  Signal { signal: i32 },
  Utf8 { utf8_error: string::FromUtf8Error },
  Io { io_error: io::Error },
  Csv { csv_error: csv::Error },
}

impl From<io::Error> for Error {
  fn from(io_error: io::Error) -> Error {
    Error::Io { io_error }
  }
}

impl From<string::FromUtf8Error> for Error {
  fn from(utf8_error: string::FromUtf8Error) -> Error {
    Error::Utf8 { utf8_error }
  }
}

impl From<csv::Error> for Error {
  fn from(csv_error: csv::Error) -> Error {
    Error::Csv { csv_error }
  }
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
    match self {
      Error::Status { status, stderr } => {
        write!(f, "Process exited with status code {}: {}", status, stderr)
      },
      Error::Io { io_error } => write!(f, "Error executing script: {}", io_error),
      Error::Signal { signal } => write!(f, "Process terminated by signal {}", signal),
      Error::Csv { csv_error } => write!(f, "Failed to deserialize CSV record: {}", csv_error),
      Error::Utf8 { utf8_error } => write!(
        f,
        "Could not convert process stdout to UTF-8: {}",
        utf8_error
      ),
    }
  }
}
