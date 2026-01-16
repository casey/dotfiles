use crate::common::*;

pub(crate) struct Library {
  base: PathBuf,
  mp3: PathBuf,
  new: PathBuf,
}

impl Library {
  #[throws]
  pub(crate) fn new() -> Self {
    let base = dirs::home_dir()
      .ok_or_else(|| anyhow!("Failed to find home dir"))?
      .join("Dropbox/music");
    Self::check_dir(&base)?;

    let new = base.join("new");
    Self::check_dir(&new)?;

    let mp3 = base.join("mp3");
    Self::check_dir(&mp3)?;

    let library = Self { base, new, mp3 };

    library
  }

  pub(crate) fn base_dir(&self) -> &Path {
    &self.base
  }

  pub(crate) fn new_dir(&self) -> &Path {
    &self.new
  }

  pub(crate) fn mp3_dir(&self) -> &Path {
    &self.mp3
  }

  pub(crate) fn mp3_path(&self, mp3: Mp3) -> PathBuf {
    self.mp3_dir().join(mp3.file_name())
  }

  #[throws]
  pub(crate) fn mp3s(&self) -> BTreeSet<Mp3> {
    Self::ids(&self.mp3, "mp3")?
      .into_iter()
      .map(Mp3::from_id)
      .collect()
  }

  #[throws]
  pub(crate) fn next_id(&self) -> Id {
    Self::ids(&self.mp3, "mp3")?
      .iter()
      .last()
      .map(|id| id.next())
      .unwrap_or_else(|| Id::new(0))
  }

  #[throws]
  fn ids(dir: &Path, expected_extension: &str) -> BTreeSet<Id> {
    let mut ids = BTreeSet::new();

    for result in WalkDir::new(dir) {
      let entry = result?;

      if entry.depth() == 0 {
        continue;
      }

      let path = entry.path().to_path_buf();

      if entry.file_type().is_dir() {
        bail!(
          "Unexpected directory in `{}`: {}",
          dir.display(),
          path.display()
        );
      }

      let file_name = path
        .file_name()
        .ok_or_else(|| anyhow!("Missing filename: `{}`", path.display()))?
        .to_string_lossy()
        .into_owned();

      if file_name == ".DS_Store" {
        continue;
      }

      let (id, extension) = file_name.split_at(6);

      let id = id
        .parse::<u32>()
        .with_context(|| anyhow!("Invalid ID: {}", path.display()))?;

      let extension = &extension[1..];

      if extension != expected_extension {
        bail!("Invalid extension: {}", path.display());
      }

      ids.insert(Id::new(id));
    }

    ids
  }

  #[throws]
  fn check_dir(path: &Path) {
    if !(path.exists() && path.is_dir()) {
      bail!("Library directory does not exist: `{}`", path.display());
    }
  }
}
