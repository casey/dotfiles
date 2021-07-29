use std::{
  env,
  os::unix::process::CommandExt,
  process::{self, Command},
};

use nvim_listen_path::nvim_listen_path;

fn main() {
  if let Err(message) = run() {
    eprintln!("error: {}", message);
    process::exit(1);
  }
}

fn run() -> Result<(), String> {
  let mut cmd = String::new();

  for arg in env::args().skip(1).rev() {
    let components = arg.split(':').collect::<Vec<&str>>();

    if components.len() != 3 {
      return Err(format!(
        "Arguments must be in the form `FILE:LINE:COLUMN`: `{}`",
        arg
      ));
    }

    cmd.push_str(&format!(
      "<esc>:edit {}<cr>{}G{}|",
      components[0], components[1], components[2]
    ));
  }

  let error = Command::new("nvr")
    .arg("-s")
    .arg("--nostart")
    .arg("--servername")
    .arg(nvim_listen_path()?)
    .arg("--remote-send")
    .arg(cmd)
    .exec();

  Err(format!("Failed to exec nvr: {}", error))
}
