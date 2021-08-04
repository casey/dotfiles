use crate::common::*;

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum Spec {
  StartingDir,
  Contains(String),
}

impl Spec {
  pub(crate) fn check(&self, context: &Context, path: &Path) -> bool {
    match self {
      Self::Contains(string) => path.join(string).exists(),
      Self::StartingDir => path == context.starting_dir,
    }
  }
}
