use crate::common::*;

pub(crate) fn windows() -> Result<Vec<WindowInfo>, Error> {
  let child = Command::new("yabai")
    .arg("-m")
    .arg("query")
    .arg("--windows")
    .stdout(Stdio::piped())
    .spawn()?;

  Ok(serde_json::from_reader(child.stdout.unwrap())?)
}
