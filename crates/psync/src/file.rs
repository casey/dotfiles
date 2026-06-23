use super::*;

#[derive(Debug, PartialEq)]
pub(crate) struct File {
  pub(crate) path: String,
  pub(crate) size: u64,
}

impl File {
  pub(crate) fn list(source: &str) -> Result<Vec<File>> {
    let output = Command::new("rsync")
      .arg("--list-only")
      .arg("--recursive")
      .arg(source)
      .output()
      .context(error::Listing)?;

    ensure!(
      output.status.success(),
      error::ListingStatus {
        status: output.status
      }
    );

    Self::parse(&String::from_utf8_lossy(&output.stdout))
  }

  fn parse(listing: &str) -> Result<Vec<File>> {
    static REGEX: OnceLock<Regex> = OnceLock::new();

    let regex = REGEX.get_or_init(|| {
      Regex::new(r"^(\S+)\s+([\d,]+)\s+\d{4}/\d{2}/\d{2}\s+\d{2}:\d{2}:\d{2}\s(.+)$").unwrap()
    });

    let mut files = Vec::new();

    for line in listing.lines() {
      let Some(captures) = regex.captures(line) else {
        continue;
      };

      if !captures[1].starts_with('-') {
        continue;
      }

      let size = captures[2]
        .replace(',', "")
        .parse::<u64>()
        .ok()
        .context(error::ListingParse {
          line: line.to_owned(),
        })?;

      files.push(File {
        path: captures[3].to_owned(),
        size,
      });
    }

    Ok(files)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse() {
    let listing = "\
drwxr-xr-x          4,096 2024/01/02 03:04:05 .
-rw-r--r--      1,048,576 2024/01/02 03:04:05 foo
-rw-r--r--          1,234 2024/01/02 03:04:05 bar baz
lrwxrwxrwx              3 2024/01/02 03:04:05 link -> foo";

    assert_eq!(
      File::parse(listing).unwrap(),
      vec![
        File {
          path: "foo".into(),
          size: 1_048_576,
        },
        File {
          path: "bar baz".into(),
          size: 1_234,
        },
      ],
    );
  }
}
