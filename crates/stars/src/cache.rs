use super::*;

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub(crate) struct Cache {
  pub(crate) stars: Vec<String>,
}

impl Cache {
  pub(crate) fn load(path: &Utf8Path) -> Result<Self> {
    match fs::read_to_string(path) {
      Ok(json) => serde_json::from_str(&json).context(error::CacheDeserialize { path }),
      Err(source) if source.kind() == io::ErrorKind::NotFound => Ok(Self::default()),
      Err(source) => Err(source).context(error::CacheRead { path }),
    }
  }

  pub(crate) fn save(&self, path: &Utf8Path) -> Result {
    let json = serde_json::to_string(self).context(error::Serialize)?;
    fs::write(path, json).context(error::CacheWrite { path })
  }
}
