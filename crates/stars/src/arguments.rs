use super::*;

#[derive(Parser)]
pub(crate) struct Arguments {
  repo: Repo,
}

impl Arguments {
  pub(crate) fn run(self) -> Result {
    let client = Client::new()?;

    let cache_dir = dirs::cache_dir().context(error::CacheDir)?;

    let cache_dir =
      Utf8PathBuf::from_path_buf(cache_dir).map_err(|path| error::PathUnicode { path }.build())?;

    let dir = cache_dir.join("stars").join(&self.repo.owner);

    fs::create_dir_all(&dir).context(error::CreateDir { path: dir.clone() })?;

    let cache_path = dir.join(format!("{}.json", self.repo.name));

    let mut cache = Cache::load(&cache_path)?;

    cache.stars.truncate(cache.stars.len() / 100 * 100);

    let count = client.stargazer_count(&self.repo)?;

    let bar = ProgressBar::new(count.max(cache.stars.len() as u64));

    bar.set_style(
      ProgressStyle::with_template(
        "{bar:40.green/white.dim} {pos:>7}/{len:<7} {percent:>3}% {eta:>6}",
      )
      .context(error::Style)?
      .progress_chars("=>-"),
    );

    bar.set_position(cache.stars.len() as u64);

    let truncated = loop {
      let page = cache.stars.len() as u64 / 100 + 1;

      if page > 400 {
        break true;
      }

      let stars = client.stargazers(&self.repo, page)?;

      let full = stars.len() == 100;

      bar.inc(stars.len() as u64);

      cache.stars.extend(stars);

      cache.save(&cache_path)?;

      if !full {
        break false;
      }
    };

    bar.finish();

    if truncated {
      eprintln!("warning: chart truncated to 40000 stars, the GitHub API listing limit");
    }

    let html_path = dir.join(format!("{}.html", self.repo.name));

    fs::write(
      &html_path,
      PageHtml::new(self.repo, &cache.stars)?.to_string(),
    )
    .context(error::HtmlWrite {
      path: html_path.clone(),
    })?;

    open::that(html_path.as_str()).context(error::Open {
      path: html_path.clone(),
    })?;

    Ok(())
  }
}
