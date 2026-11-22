use std::io;

fn main() -> io::Result<()> {
  print!("{}", json(&io::read_to_string(io::stdin())?));
  Ok(())
}

fn entry(line: &str) -> Option<(&str, &str)> {
  let (_, comment) = line.split_once("; fully-qualified")?;
  let (_, comment) = comment.split_once("# ")?;
  let mut fields = comment.splitn(3, ' ');
  let emoji = fields.next()?;
  fields.next()?;
  let name = fields.next()?;

  if emoji
    .chars()
    .any(|c| ('\u{1F3FB}'..='\u{1F3FF}').contains(&c))
  {
    return None;
  }

  Some((emoji, name))
}

fn json(test: &str) -> String {
  let entries = test
    .lines()
    .filter(|line| !line.starts_with('#'))
    .filter_map(entry)
    .map(|entry| serde_json::to_string(&entry).unwrap())
    .collect::<Vec<String>>();

  format!("[\n{}\n]\n", entries.join(",\n"))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn conversion() {
    assert_eq!(
      json(
        "# fully-qualified : 1
1F62D       ; fully-qualified     # 😭 E0.6 foo bar
1F44D 1F3FD ; fully-qualified     # 👍🏽 E1.0 baz
263A        ; unqualified         # ☺ E0.6 qux
"
      ),
      "[\n[\"😭\",\"foo bar\"]\n]\n",
    );
  }
}
