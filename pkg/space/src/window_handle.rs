use crate::common::*;

pub(crate) struct WindowHandle {
  pub(crate) id: u64,
}

impl WindowHandle {
  pub(crate) fn set_floating(&self, floating: usize) -> Result<(), Error> {
    let info = self.query()?;

    if info.floating != floating {
      let exit_status = Command::new("yabai")
        .arg("-m")
        .arg("window")
        .arg(self.id.to_string())
        .arg("--toggle")
        .arg("float")
        .status()?;

      if !exit_status.success() {
        return Err(Error::ExitStatus { exit_status });
      }
    }

    Ok(())
  }

  pub(crate) fn query(&self) -> Result<WindowInfo, Error> {
    let child = Command::new("yabai")
      .arg("-m")
      .arg("query")
      .arg("--windows")
      .arg("--window")
      .arg(self.id.to_string())
      .stdout(Stdio::piped())
      .spawn()?;

    Ok(serde_json::from_reader(child.stdout.unwrap())?)
  }

  pub(crate) fn swap(&self, other: &WindowHandle) -> Result<(), Error> {
    // yabai -m window self.id.to_string() --swap other.id.to_string()
    unimplemented!()
  }
}
