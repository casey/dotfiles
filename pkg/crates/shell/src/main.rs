mod common;
mod error;
mod run;
mod session;

use crate::common::*;

fn main() -> Result<(), Error> {
  let output = run(
    r##"
    #!/usr/bin/env bash

    echo "name,attached"

    tmux list-sessions -F '#{session_name},#{session_attached}'

    exit 0
    "##,
  )?;

  let mut reader = csv::Reader::from_reader(Cursor::new(output));

  let sessions = reader
    .deserialize::<Session>()
    .into_iter()
    .collect::<Result<Vec<Session>, csv::Error>>()?;

  let mut shell = Command::new("tmux");

  if let Some(detached) = sessions
    .iter()
    .filter(|session| session.is_detached())
    .next()
  {
    shell.arg("attach-session");
    shell.arg("-t");
    shell.arg(&detached.name);
  } else {
    let names = sessions.iter().map(Session::name).collect::<Vec<&str>>();

    for i in 0..u64::MAX {
      let name = i.to_string();

      if !names.contains(&name.as_str()) {
        shell.arg("new-session");
        shell.arg("-s");
        shell.arg(name);
        break;
      }
    }
  }

  let error = shell.exec();

  Err(error.into())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn simple() {
    assert_eq!(run("#!/usr/bin/env bash\necho hello").unwrap(), "hello\n");
  }
}
