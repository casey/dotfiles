use crate::common::*;

pub fn run(script: &str) -> Result<String, Error> {
  let tempdir = tempfile::tempdir()?;

  let path = tempdir.path().join("script");

  fs::write(&path, script)?;

  let mut permissions = fs::metadata(&path)?.permissions();

  let current_mode = permissions.mode();
  permissions.set_mode(current_mode | 0o100);

  fs::set_permissions(&path, permissions)?;

  let output = Command::new(&path).output()?;

  let stdout = String::from_utf8(output.stdout)?;

  let stderr = String::from_utf8(output.stderr)?;

  let status = output.status;

  if !status.success() {
    if let Some(status) = status.code() {
      if status != 0 {
        return Err(Error::Status { status, stderr });
      }
    } else if let Some(signal) = status.signal() {
      return Err(Error::Signal { signal });
    }
  }

  Ok(stdout)
}
