use {
  self::{
    arguments::Arguments, cache::Cache, error::Error, github::Client, page::PageHtml, repo::Repo,
  },
  boilerplate::{Boilerplate, Trusted},
  camino::{Utf8Path, Utf8PathBuf},
  clap::Parser,
  indicatif::{ProgressBar, ProgressStyle, style::TemplateError},
  serde::{Deserialize, Serialize},
  snafu::{ErrorCompat, OptionExt, ResultExt, Snafu, ensure},
  std::{
    backtrace::{Backtrace, BacktraceStatus},
    env,
    fmt::{self, Display, Formatter},
    fs, io,
    path::PathBuf,
    process::{Command, ExitCode, ExitStatus},
    str::FromStr,
    thread,
    time::Duration,
  },
  ureq::{Agent, Body, http::Response},
};

mod arguments;
mod cache;
mod error;
mod github;
mod page;
mod repo;

type Result<T = (), E = Error> = std::result::Result<T, E>;

fn main() -> ExitCode {
  if let Err(error) = Arguments::parse().run() {
    eprintln!("error: {error}");

    for cause in Error::iter_chain(&error).skip(1) {
      eprintln!("because: {cause}");
    }

    if let Some(backtrace) = Error::backtrace(&error)
      && backtrace.status() == BacktraceStatus::Captured
    {
      eprintln!();
      eprintln!("backtrace:");
      eprintln!("{backtrace}");
    }

    return ExitCode::FAILURE;
  }

  ExitCode::SUCCESS
}
