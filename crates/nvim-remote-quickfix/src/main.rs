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
    let mut errors = String::from("[");

    for location in env::args().skip(1).collect::<Vec<String>>() {
        errors.push_str(&format!("'{}', ", location));
    }

    errors.push_str("]");

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
        .arg(format!("call QuickfixPopulate({})", errors))
        .exec();

    Err(format!("Failed to exec nvr: {}", error))
}
