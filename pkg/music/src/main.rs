use crate::common::*;

mod common;
mod error;
mod fs;
mod import;
mod library;
mod subcommand;
mod track;

fn main() -> Result<()> {
  Subcommand::from_args().run()
}
