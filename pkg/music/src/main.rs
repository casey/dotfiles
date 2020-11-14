use crate::common::*;

mod cluster;
mod cluster_key;
mod clusterizer;
mod common;
mod flac;
mod format;
mod fs;
mod id;
mod import;
mod import_key;
mod library;
mod mp3;
mod subcommand;

fn main() {
  if let Err(err) = Subcommand::from_args().run() {
    eprintln!("error: {}", err);
    eprintln!();
    for because in err.chain().skip(1) {
      eprintln!("∵ {}", because);
    }
    process::exit(1);
  }
}
