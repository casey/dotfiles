use std::{
  env,
  os::unix::process::CommandExt,
  path::Path,
  process::{self, Command},
};

fn main() {
  if let Err(message) = run() {
    eprintln!("error: {}", message);
    process::exit(1);
  }
}

fn run() -> Result<(), String> {
  let mut cmd = String::from("cgetexpr [");

  for location in env::args().skip(1).rev().collect::<Vec<String>>() {
    cmd.push_str(&format!("'{}', ", location));
  }

  cmd.push(']');

  let tmpdir = env::var("TMPDIR")
    .map_err(|error| format!("Failed to get `TMPDIR` environment variable: {}", error))?;

  let project_root = project_root::from_current_dir()
    .map_err(|error| format!("Failed to get project root: {}", error))?;

  let listen_path = Path::new(&tmpdir).join(format!(
    "nvim-{}",
    project_root.trim_matches('/').replace('/', "%")
  ));

  let error = Command::new("nvr")
    .arg("-s")
    .arg("--nostart")
    .arg("--servername")
    .arg(listen_path)
    .arg("-c")
    .arg(cmd)
    .exec();

  Err(format!("Failed to exec nvr: {}", error))
}
