use super::*;

const ECHARTS: &str = include_str!(concat!(env!("OUT_DIR"), "/echarts.min.js"));

#[derive(Boilerplate)]
pub(crate) struct PageHtml {
  data: String,
  rate: String,
  repo: Repo,
}

impl PageHtml {
  pub(crate) fn new(repo: Repo, stars: &[String]) -> Result<Self> {
    Ok(Self {
      data: serde_json::to_string(&series(stars)).context(error::Serialize)?,
      rate: serde_json::to_string(&rate(stars)?).context(error::Serialize)?,
      repo,
    })
  }
}

fn series(stars: &[String]) -> Vec<(&str, u64)> {
  stars
    .iter()
    .enumerate()
    .map(|(i, star)| (star.as_str(), i as u64 + 1))
    .collect()
}

fn rate(stars: &[String]) -> Result<Vec<(String, f64)>> {
  let mut dates = stars
    .iter()
    .map(|star| {
      DateTime::parse_from_rfc3339(star)
        .map(|date| date.date_naive())
        .context(error::StarParse { star })
    })
    .collect::<Result<Vec<NaiveDate>>>()?;

  dates.sort();

  let (Some(first), Some(last)) = (dates.first(), dates.last()) else {
    return Ok(Vec::new());
  };

  let span = (*last - *first).num_days() as u64 + 1;

  let width = span.div_ceil(100);

  let mut buckets = vec![0u64; span.div_ceil(width) as usize];

  for date in &dates {
    buckets[((*date - *first).num_days() as u64 / width) as usize] += 1;
  }

  Ok(
    buckets
      .into_iter()
      .enumerate()
      .map(|(i, count)| {
        let offset = i as u64 * width;
        let start = *first + Days::new(offset);
        let days = width.min(span - offset);
        (start.to_string(), count as f64 / days as f64)
      })
      .collect(),
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  fn stars(stars: &[&str]) -> Vec<String> {
    stars.iter().map(|star| star.to_string()).collect()
  }

  #[test]
  fn rate_empty() {
    assert_eq!(rate(&[]).unwrap(), Vec::new());
  }

  #[test]
  fn rate_daily_buckets() {
    assert_eq!(
      rate(&stars(&[
        "2020-01-03T12:00:00Z",
        "2020-01-01T00:00:00Z",
        "2020-01-01T23:59:59Z",
      ]))
      .unwrap(),
      vec![
        ("2020-01-01".into(), 2.0),
        ("2020-01-02".into(), 0.0),
        ("2020-01-03".into(), 1.0),
      ],
    );
  }

  #[test]
  fn rate_wide_buckets() {
    assert_eq!(
      rate(&stars(&[
        "2020-01-01T00:00:00Z",
        "2020-01-01T00:00:00Z",
        "2020-01-01T00:00:00Z",
        "2020-01-03T00:00:00Z",
        "2020-01-03T00:00:00Z",
        "2020-01-03T00:00:00Z",
        "2020-07-20T00:00:00Z",
      ]))
      .unwrap()
      .into_iter()
      .filter(|(_, rate)| *rate > 0.0)
      .collect::<Vec<(String, f64)>>(),
      vec![("2020-01-01".into(), 2.0), ("2020-07-20".into(), 1.0)],
    );
  }

  #[test]
  fn rate_invalid_star() {
    assert_eq!(
      rate(&stars(&["foo"])).unwrap_err().to_string(),
      "failed to parse star timestamp `foo`",
    );
  }
}
