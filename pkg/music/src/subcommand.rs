use crate::common::*;

#[derive(StructOpt)]
pub(crate) enum Subcommand {
  Import,
  Transcode,
}

impl Subcommand {
  #[throws]
  pub(crate) fn run(self) {
    let library = Library::new()?;

    let appender = tracing_appender::rolling::never(library.base_dir(), "log.txt");

    let (non_blocking, _guard) = tracing_appender::non_blocking(appender);

    let filter = EnvFilter::from_default_env()
      .add_directive("music=info".parse()?)
      .add_directive("warn".parse()?);

    let subscriber = tracing_subscriber::registry()
      .with(filter)
      .with(tracing_subscriber::fmt::Layer::new())
      .with(
        tracing_subscriber::fmt::Layer::new()
          .json()
          .with_writer(non_blocking),
      );

    tracing_log::LogTracer::init()?;

    tracing::subscriber::set_global_default(subscriber)?;

    match self {
      Self::Import => Self::import(&library)?,
      Self::Transcode => Self::transcode(&library)?,
    }
  }

  #[throws]
  fn import(library: &Library) {
    let span = span!(Level::INFO, "import");
    let _guard = span.enter();

    info!("Looking for new tracks…");

    let mut mp3s = 0;
    let mut flacs = 0;
    let mut others = 0;
    let mut clusterizer = Clusterizer::new();

    for result in WalkDir::new(library.new_dir()) {
      let entry = result?;

      if entry.file_type().is_dir() {
        continue;
      }

      let path = entry.path().to_owned();

      let extension = path
        .extension()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned();

      match extension.as_str() {
        "mp3" => {
          clusterizer.insert(&path)?;
          mp3s += 1;
        },
        "flac" => {
          clusterizer.insert(&path)?;
          flacs += 1;
        },
        _ => others += 1,
      };
    }

    info!("Found {} MP3s.", mp3s);
    info!("Found {} FLACs.", flacs);
    info!("Found {} other files.", others);

    let next_id = library.next_id()?;

    info!("Next ID: {}", next_id);

    clusterizer.check()?;

    {
      let mut next_id = next_id;
      for cluster in clusterizer.clusters() {
        for import in cluster.imports() {
          let destination = import.destination(library, next_id);
          if destination.exists() {
            bail!("Import destination exists: `{}`", destination.display());
          }
          next_id = next_id.next();
        }
      }
    }

    info!("Found {} clusters:", clusterizer.cluster_count());

    if clusterizer.cluster_count() > 0 {
      for cluster_key in clusterizer.keys() {
        info!("- {}", cluster_key);
      }

      let input: bool = input().msg("Do these clusters look okay? ").get();

      if !input {
        return;
      }

      {
        let mut next_id = next_id;
        for cluster in clusterizer.clusters() {
          for import in cluster.imports() {
            let destination = import.destination(library, next_id);
            fs::rename(&import.path(), &destination)?;
            next_id = next_id.next();
          }
        }
      }
    }

    Self::transcode(library)?;
  }

  #[throws]
  fn transcode(library: &Library) {
    let span = span!(Level::INFO, "transcode");
    let _guard = span.enter();

    let mut flacs = library.flacs()?;

    for mp3 in library.mp3s()? {
      flacs.remove(&Flac::from_id(mp3.id()));
    }

    let total = flacs.len();
    let total_width = total.to_string().len();
    info!("{} FLACs to transcode…", total);

    let tmpdir = TempDir::new().unwrap();

    let i = AtomicUsize::new(0);

    let results = flacs
      .par_iter()
      .map(|flac| {
        let mp3 = Mp3::from_id(flac.id());
        let src = library.flac_path(*flac);
        let tmp = tmpdir.path().join(mp3.file_name());
        let dst = library.mp3_path(mp3);

        Command::new("ffmpeg")
          .arg("-i")
          .arg(&src)
          .arg("-qscale:a")
          .arg("0")
          .arg(&tmp)
          .output()
          .with_context(|| anyhow!("Failed to invoke ffmpeg"))
          .and_then(|output| {
            let i = i.fetch_add(1, Ordering::Relaxed);
            let count = format!("{:width$}/{}", i + 1, total, width = total_width);
            if output.status.success() {
              info!("{} [+] {}", count, flac.file_name());
              Ok((src, tmp, dst))
            } else {
              error!("{} [x] {}", count, flac.file_name());
              Err(anyhow!(
                "Transcoding failed: {}:\n{}",
                output.status,
                String::from_utf8_lossy(&output.stderr).into_owned()
              ))
            }
          })
      })
      .collect::<Vec<Result<(PathBuf, PathBuf, PathBuf)>>>();

    let mut records = Vec::new();

    for result in results {
      match result {
        Err(err) => bail!("{}", err),
        Ok(record) => records.push(record),
      }
    }

    for (_, tmp, dst) in records {
      info!("Renaming {:?} to {:?}", tmp, &dst);
      fs::rename(&tmp, &dst)?;
    }
  }
}
