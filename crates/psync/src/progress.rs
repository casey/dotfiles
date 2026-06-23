use super::*;

pub(crate) fn parse(line: &str) -> Option<u64> {
  static REGEX: OnceLock<Regex> = OnceLock::new();

  let regex = REGEX.get_or_init(|| Regex::new(r"^\s*([\d,]+)\s+\d+%").unwrap());

  regex
    .captures(line)?
    .get(1)?
    .as_str()
    .replace(',', "")
    .parse::<u64>()
    .ok()
}

#[cfg(test)]
mod tests {
  #[test]
  fn parse() {
    #[track_caller]
    fn case(line: &str, expected: Option<u64>) {
      assert_eq!(super::parse(line), expected);
    }

    case(
      "       1,048,576  50%    1.00MB/s    0:00:10",
      Some(1_048_576),
    );
    case(
      "       1,234,567 100%   12.34MB/s    0:00:01 (xfr#1, to-chk=0/1)",
      Some(1_234_567),
    );
    case("foo/bar.bin", None);
    case("", None);
  }
}
