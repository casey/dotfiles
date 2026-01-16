use crate::common::*;

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum Spec {
  StartingDir,
  Contains(String),
}

impl Spec {
  pub(crate) fn check(&self, context: &Context, dir: &Path) -> Result<bool, Error> {
    match self {
      Self::Contains(file_name) => {
        for result in fs::read_dir(&dir).context(error::FilesystemIo { path: dir })? {
          let entry = result.context(error::FilesystemIo { path: dir })?;
          if entry
            .file_name()
            .to_string_lossy()
            .eq_ignore_ascii_case(file_name)
          {
            return Ok(true);
          }
        }
        return Ok(false);
      }
      Self::StartingDir => Ok(dir == context.starting_dir),
    }
  }
}
