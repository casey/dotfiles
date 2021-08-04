use crate::common::*;

mod common;
mod config;
mod context;
mod error;
mod spec;

pub fn from_current_dir() -> Result<String, Error> {
  from_starting_dir(&env::current_dir().context(error::CurrentDir)?)
}

pub fn from_starting_dir(starting_dir: &Path) -> Result<String, Error> {
  let home_dir = dirs::home_dir().ok_or(Error::HomeDir)?;

  let config_path = home_dir.join(".project-root.yaml");

  let yaml =
    fs::read_to_string(&config_path).context(error::FilesystemIo { path: &config_path })?;

  let config = serde_yaml::from_str::<Config>(&yaml)
    .context(error::ConfigDeserialize { path: &config_path })?;

  let context = Context {
    starting_dir: starting_dir.to_owned(),
  };

  for group in config.specs {
    for dir in starting_dir.ancestors() {
      for spec in &group {
        if spec.check(&context, dir) {
          return Ok(
            dir
              .to_str()
              .ok_or_else(|| Error::PathDecode {
                path: dir.to_owned(),
              })?
              .to_owned(),
          );
        }
      }
    }
  }

  Err(Error::NotFound)
}
