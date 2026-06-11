use super::*;

#[derive(Parser)]
#[command(version)]
pub(crate) enum Arguments {
  #[command(about = "Add PR and contributor links to changelog")]
  UpdateContributors,
}

impl Arguments {
  pub(crate) fn run(self) -> Result<(), Error> {
    match self {
      Self::UpdateContributors => Self::update_contributors(),
    }
  }

  fn command(command: &str, args: &[&str]) -> Result<String, Error> {
    let output = Command::new(command).args(args).output();

    let command = format!("{command} {}", args.join(" "));

    let output = output.context(error::CommandInvocation { command: &command })?;

    let stderr =
      str::from_utf8(&output.stderr).context(error::CommandOutput { command: &command })?;

    ensure!(
      output.status.success(),
      error::CommandStatus {
        command: &command,
        stderr
      }
    );

    let stdout =
      str::from_utf8(&output.stdout).context(error::CommandOutput { command: &command })?;

    Ok(stdout.trim().into())
  }

  fn update_contributors() -> Result<(), Error> {
    let repo = Self::command(
      "gh",
      &[
        "repo",
        "view",
        "--json",
        "nameWithOwner",
        "--jq",
        ".nameWithOwner",
      ],
    )?;

    let changelog = fs::read_to_string("CHANGELOG.md").context(error::ReadChangelog)?;

    let regex = Regex::new(r"\(#(\d+)( by @[a-z]+)?\)").unwrap();

    let mut contributors = HashMap::new();

    for captures in regex.captures_iter(&changelog) {
      if let Entry::Vacant(entry) = contributors.entry(captures[1].to_owned()) {
        let pr = entry.key().as_str();
        eprintln!("#{pr}");
        let contributor = Self::command(
          "gh",
          &[
            "pr",
            "view",
            pr,
            "--json",
            "author",
            "--jq",
            ".author.login",
          ],
        )?;
        entry.insert(contributor);
      }
    }

    let changelog = regex.replace_all(&changelog, |captures: &Captures| {
      let pr = &captures[1];
      let contributor = &contributors[pr];
      format!(
        "([#{pr}](https://github.com/{repo}/pull/{pr}) by [{contributor}](https://github.com/{contributor}))"
      )
    });

    fs::write("CHANGELOG.md", &*changelog).context(error::WriteChangelog)
  }
}
