use super::*;

#[derive(Parser)]
pub(crate) struct Arguments {
  #[arg(long)]
  compress: bool,
  #[arg(long, default_value_t = 1 << 20)]
  minimum_size: u64,
  #[arg(long, default_value_t = 4)]
  threads: usize,
  source: String,
  destination: Utf8PathBuf,
}

impl Arguments {
  pub(crate) fn run(self) -> Result {
    let source = if self.source.ends_with('/') {
      self.source.clone()
    } else {
      format!("{}/", self.source)
    };

    fs::create_dir_all(&self.destination).context(error::CreateDir {
      path: self.destination.clone(),
    })?;

    let mut files = File::list(&source)?
      .into_iter()
      .filter(|file| file.size >= self.minimum_size)
      .collect::<Vec<File>>();

    files.sort_by_key(|file| std::cmp::Reverse(file.size));

    let total = files.iter().map(|file| file.size).sum::<u64>();

    let multi = MultiProgress::new();

    let overall = multi.add(ProgressBar::new(total));
    overall.set_style(
      ProgressStyle::with_template(
        "total {bar:40} {bytes:>11}/{total_bytes:<11} {percent:>3}% {binary_bytes_per_sec:>11} {eta:>6}",
      )
      .context(error::Style)?
      .progress_chars("=>-"),
    );

    let files = Arc::new(files);
    let cursor = Arc::new(AtomicUsize::new(0));
    let failed = Arc::new(AtomicBool::new(false));

    let mut handles = Vec::new();

    for _ in 0..self.threads {
      let bar = multi.add(ProgressBar::new(0));
      bar.set_style(
        ProgressStyle::with_template(
          "{bar:30} {bytes:>11}/{total_bytes:<11} {binary_bytes_per_sec:>11} {wide_msg}",
        )
        .context(error::Style)?
        .progress_chars("=>-"),
      );

      let worker = Worker {
        bar,
        compress: self.compress,
        cursor: cursor.clone(),
        destination: self.destination.clone(),
        failed: failed.clone(),
        files: files.clone(),
        overall: overall.clone(),
        source: source.clone(),
      };

      handles.push(thread::spawn(move || worker.run()));
    }

    let mut result = Ok(());

    for handle in handles {
      let joined = match handle.join() {
        Ok(joined) => joined,
        Err(_) => Err(error::Thread.build()),
      };

      if let Err(error) = joined
        && result.is_ok()
      {
        result = Err(error);
      }
    }

    overall.finish();

    result
  }
}
