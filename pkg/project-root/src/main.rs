use std::{path::PathBuf, process};

use structopt::StructOpt;

#[derive(StructOpt)]
struct Arguments {
  starting_dir: Option<PathBuf>,
}

fn main() {
  let arguments = Arguments::from_args();

  match project_root::project_root(arguments.starting_dir.as_deref()) {
    Ok(project_root) => println!("{}", project_root),
    Err(error) => {
      eprintln!("error: {}", error);
      process::exit(1);
    },
  }
}
