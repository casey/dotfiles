use crate::common::*;

use std::collections::btree_map::Values;

#[derive(Default)]
pub(crate) struct Cluster {
  imports: BTreeMap<ImportKey, Import>,
}

impl Cluster {
  #[throws]
  pub(crate) fn insert(&mut self, import: Import) {
    let key = (&import).into();
    if self.imports.contains_key(&key) {
      bail!("Duplicate track while clusterizing: {:?}", import)
    }
    self.imports.insert(key, import)
  }

  pub(crate) fn imports(&self) -> Values<ImportKey, Import> {
    self.imports.values()
  }

  #[throws]
  pub(crate) fn check(&self) {
    if self.imports.is_empty() {
      bail!("Empty cluster!");
    }

    let mut disc = 0;
    let mut track = 0;

    for (key, import) in &self.imports {
      if key.disc_number() != disc {
        if key.disc_number() == disc + 1 {
          disc += 1;
          track = 0;
        } else {
          bail!("Missing disc before import: `{}`", import.path().display());
        }
      }

      if key.track_number() != track {
        if key.track_number() == track + 1 {
          track += 1;
        } else {
          bail!("Missing track before import: `{}`", import.path().display());
        }
      }
    }
  }
}
