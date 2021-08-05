use std::{path::PathBuf, process};

use structopt::StructOpt;

#[derive(StructOpt)]
struct Arguments {
  starting_dir: Option<PathBuf>,
}

fn main() {
  let arguments = Arguments::from_args();

  let result = match arguments.starting_dir {
    Some(starting_dir) => project_root::from_starting_dir(&starting_dir),
    None => project_root::from_current_dir(),
  };

  match result {
    Ok(project_root) => println!("{}", project_root),
    Err(error) => {
      eprintln!("error: {}", error);
      process::exit(1);
    },
  }
}
