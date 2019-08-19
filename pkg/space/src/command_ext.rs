use crate::common::*;

pub trait CommandExt {
  fn deserialize_json<T: DeserializeOwned>(self) -> Result<T, Error>;
}

impl CommandExt for Command {
  fn deserialize_json<T: DeserializeOwned>(mut self) -> Result<T, Error> {
    self.stdin(Stdio::null());
    self.stdout(Stdio::piped());
    self.stderr(Stdio::piped());

    let child = self.spawn().map_err(|


  let child = Command::new("yabai")
    .arg("-m")
    .arg("query")
    .arg("--windows")
    .stdout(Stdio::piped())
    .spawn()?;

  Ok(serde_json::from_reader(child.stdout.unwrap())?)


  }
}
