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
  let nvim_listen_path = nvim_listen_path()?;

  let error = Command::new("nvim")
    .arg("--listen")
    .arg(nvim_listen_path)
    .args(env::args().skip(1))
    .exec();

  Err(format!("Failed to exec nvim: {}", error))
}
