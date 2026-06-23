use {
  self::{arguments::Arguments, error::Error, file::File, worker::Worker},
  camino::Utf8PathBuf,
  clap::Parser,
  indicatif::{MultiProgress, ProgressBar, ProgressStyle, style::TemplateError},
  regex::Regex,
  snafu::{ErrorCompat, OptionExt, ResultExt, Snafu, ensure},
  std::{
    backtrace::{Backtrace, BacktraceStatus},
    fs,
    io::{self, BufReader, Read, Write},
    process::{Command, ExitCode, ExitStatus, Stdio},
    sync::{
      Arc, OnceLock,
      atomic::{AtomicBool, AtomicUsize, Ordering},
    },
    thread,
  },
};

mod arguments;
mod error;
mod file;
mod progress;
mod worker;

type Result<T = (), E = Error> = std::result::Result<T, E>;

const THREADS: usize = 4;
const MINIMUM_SIZE: u64 = 1 << 20;

pub fn run() -> ExitCode {
  if let Err(error) = Arguments::parse().run() {
    eprintln!("error: {error}");

    for cause in ErrorCompat::iter_chain(&error).skip(1) {
      eprintln!("because: {cause}");
    }

    if let Some(backtrace) = ErrorCompat::backtrace(&error)
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
