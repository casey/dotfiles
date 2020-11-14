/// stdlib
pub(crate) use std::{
  collections::{BTreeMap, BTreeSet},
  convert::{TryFrom, TryInto},
  fmt::{self, Display, Formatter},
  path::{Path, PathBuf},
  process::{self, Command},
  sync::atomic::{AtomicUsize, Ordering},
};

/// dependencies
pub(crate) use ::{
  anyhow::{anyhow, bail, Context, Error, Result},
  fehler::throws,
  rayon::iter::{IntoParallelRefIterator, ParallelIterator},
  read_input::{shortcut::input, InputBuild},
  structopt::StructOpt,
  tempfile::TempDir,
  tracing::{error, info, span, warn, Level},
  tracing_log::LogTracer,
  tracing_subscriber::{layer::SubscriberExt, EnvFilter},
  walkdir::WalkDir,
};

/// modules
pub(crate) use crate::fs;

/// structs and enums
pub(crate) use crate::{
  cluster::Cluster, cluster_key::ClusterKey, clusterizer::Clusterizer, flac::Flac, format::Format,
  id::Id, import::Import, import_key::ImportKey, library::Library, mp3::Mp3,
  subcommand::Subcommand,
};
