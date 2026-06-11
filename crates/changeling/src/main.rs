use {
  arguments::Arguments,
  clap::Parser,
  error::Error,
  regex::{Captures, Regex},
  snafu::{ErrorCompat, ResultExt, Snafu, ensure},
  std::{
    collections::{HashMap, hash_map::Entry},
    fs,
    io::{self, IsTerminal},
    process::{Command, ExitCode},
    str::{self, Utf8Error},
  },
};

mod arguments;
mod error;

fn main() -> ExitCode {
  if let Err(error) = Arguments::parse().run() {
    if io::stderr().is_terminal() {
      eprintln!("\x1b[1;31merror\x1b[0m: \x1b[1m{error}\x1b[0m");
    } else {
      eprintln!("error: {error}");
    }

    let causes = error.iter_chain().skip(1).count();

    for (i, source) in error.iter_chain().skip(1).enumerate() {
      eprintln!(
        "       {}─ {source}",
        if i < causes - 1 { '├' } else { '└' }
      );
    }

    ExitCode::FAILURE
  } else {
    ExitCode::SUCCESS
  }
}
