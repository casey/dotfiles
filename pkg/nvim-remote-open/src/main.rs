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

  let tmpdir = env::var("TMPDIR")
    .map_err(|error| format!("Failed to get `TMPDIR` environment variable: {}", error))?;

  let current_dir =
    env::current_dir().map_err(|error| format!("Failed to get current directory: {}", error))?;

  let current_dir = current_dir.to_str().ok_or_else(|| {
    format!(
      "Current directory not valid unicode: {}",
      current_dir.display()
    )
  })?;

  let listen_path = Path::new(&tmpdir).join(format!(
    "nvim-{}",
    current_dir.trim_matches('/').replace('/', "%")
  ));

  let error = Command::new("nvr")
    .arg("-s")
    .arg("--nostart")
    .arg("--servername")
    .arg(listen_path)
    .arg("--remote-send")
    .arg(cmd)
    .exec();

  Err(format!("Failed to exec nvr: {}", error))
}
