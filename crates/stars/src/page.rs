use super::*;

const ECHARTS: &str = include_str!(concat!(env!("OUT_DIR"), "/echarts.min.js"));

#[derive(Boilerplate)]
pub(crate) struct PageHtml {
  data: String,
  repo: Repo,
}

impl PageHtml {
  pub(crate) fn new(repo: Repo, stars: &[String]) -> Result<Self> {
    Ok(Self {
      data: serde_json::to_string(&series(stars)).context(error::Serialize)?,
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
