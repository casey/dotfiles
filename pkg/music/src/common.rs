/// stdlib
pub(crate) use std::{
  collections::{BTreeMap, BTreeSet},
  convert::{TryFrom, TryInto},
  fmt::{self, Display, Formatter},
  path::{Path, PathBuf},
  process::{self},
};

/// dependencies
pub(crate) use ::{
  anyhow::{anyhow, bail, Context, Error},
  fehler::throws,
  read_input::{shortcut::input, InputBuild},
  structopt::StructOpt,
  tracing::{info, span, trace, Level},
  tracing_log::LogTracer,
  tracing_subscriber::{layer::SubscriberExt, EnvFilter},
  walkdir::WalkDir,
};

/// modules
pub(crate) use crate::fs;

/// structs and enums
pub(crate) use crate::{
  cluster::Cluster, cluster_key::ClusterKey, clusterizer::Clusterizer, id::Id, import::Import,
  import_key::ImportKey, library::Library, mp3::Mp3, subcommand::Subcommand,
};
