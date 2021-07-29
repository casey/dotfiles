use std::{
  env,
  ffi::OsString,
  path::{Component, PathBuf},
};

pub fn nvim_listen_path() -> Result<PathBuf, String> {
  let mut filename = OsString::from("nvim-");

  for (i, component) in env::current_dir()
    .map_err(|error| format!("Failed to get current directory: {}", error))?
    .components()
    .filter_map(|component| {
      if let Component::Normal(component) = component {
        Some(component)
      } else {
        None
      }
    })
    .enumerate()
  {
    if i > 0 {
      filename.push("%");
    }
    filename.push(component);
  }

  Ok(env::temp_dir().join(filename))
}
