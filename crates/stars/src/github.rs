use super::*;

pub(crate) struct Client {
  agent: Agent,
  token: String,
}

#[derive(Deserialize)]
struct Repository {
  stargazers_count: u64,
}

#[derive(Deserialize)]
struct Star {
  starred_at: String,
}

impl Client {
  pub(crate) fn new() -> Result<Self> {
    let token = if let Ok(token) = env::var("GITHUB_TOKEN") {
      token
    } else {
      let output = Command::new("gh")
        .args(["auth", "token"])
        .output()
        .context(error::Token)?;

      ensure!(
        output.status.success(),
        error::TokenStatus {
          status: output.status
        }
      );

      String::from_utf8(output.stdout).unwrap().trim().to_string()
    };

    ensure!(!token.is_empty(), error::TokenEmpty);

    Ok(Self {
      agent: Agent::new_with_config(Agent::config_builder().http_status_as_error(false).build()),
      token,
    })
  }

  pub(crate) fn stargazer_count(&self, repo: &Repo) -> Result<u64> {
    let mut response = self.get(&format!("https://api.github.com/repos/{repo}"))?;

    let status = response.status().as_u16();

    ensure!(status != 404, error::RepoNotFound { repo: repo.clone() });

    ensure!(response.status().is_success(), error::Status { status });

    Ok(
      response
        .body_mut()
        .read_json::<Repository>()
        .context(error::Http)?
        .stargazers_count,
    )
  }

  pub(crate) fn stargazers(&self, repo: &Repo, page: u64) -> Result<Vec<String>> {
    let mut response = self.get(&format!(
      "https://api.github.com/repos/{repo}/stargazers?per_page=100&page={page}"
    ))?;

    let status = response.status().as_u16();

    ensure!(response.status().is_success(), error::Status { status });

    Ok(
      response
        .body_mut()
        .read_json::<Vec<Star>>()
        .context(error::Http)?
        .into_iter()
        .map(|star| star.starred_at)
        .collect(),
    )
  }

  fn get(&self, url: &str) -> Result<Response<Body>> {
    let mut attempts = 0;

    loop {
      attempts += 1;

      let response = self
        .agent
        .get(url)
        .header("accept", "application/vnd.github.star+json")
        .header("authorization", format!("bearer {}", self.token))
        .header("user-agent", "stars")
        .call()
        .context(error::Http)?;

      let status = response.status().as_u16();

      if (status == 403 || status == 429 || status >= 500) && attempts < 4 {
        let seconds = response
          .headers()
          .get("retry-after")
          .and_then(|value| value.to_str().ok())
          .and_then(|value| value.parse().ok())
          .unwrap_or(if status >= 500 { 2 } else { 60 });

        thread::sleep(Duration::from_secs(seconds));

        continue;
      }

      return Ok(response);
    }
  }
}
