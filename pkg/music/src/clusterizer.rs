use crate::common::*;

use std::collections::btree_map::{Keys, Values};

#[derive(Default)]
pub(crate) struct Clusterizer {
  clusters: BTreeMap<ClusterKey, Cluster>,
}

impl Clusterizer {
  pub(crate) fn new() -> Self {
    Self::default()
  }

  pub(crate) fn keys(&self) -> Keys<ClusterKey, Cluster> {
    self.clusters.keys()
  }

  pub(crate) fn clusters(&self) -> Values<ClusterKey, Cluster> {
    self.clusters.values()
  }

  #[throws]
  pub(crate) fn insert(&mut self, path: &Path) {
    let import: Import = path.try_into()?;

    self
      .clusters
      .entry((&import).into())
      .or_default()
      .insert(import)?;
  }

  pub(crate) fn cluster_count(&self) -> usize {
    self.clusters.len()
  }

  #[throws]
  pub(crate) fn check(&self) {
    for cluster in self.clusters.values() {
      cluster.check()?;
    }
  }
}
