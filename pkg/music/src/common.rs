pub(crate) use std::{
  collections::{BTreeMap, BTreeSet},
  ffi::OsStr,
  fs, io,
  num::ParseIntError,
  path::{Path, PathBuf},
};

pub(crate) use chrono::{DateTime, NaiveDateTime, Utc};
pub(crate) use id3::Tag;
pub(crate) use snafu::{ResultExt, Snafu};
pub(crate) use walkdir::{DirEntry, WalkDir};

pub(crate) use crate::{
  error::{self, Error},
  track::Track,
};

pub(crate) type Result<T, E = Error> = std::result::Result<T, E>;
