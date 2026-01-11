use crate::common::*;

#[derive(StructOpt)]
pub(crate) enum Subcommand {
  Import,
  FixTranscodeTags,
  #[structopt(alias("show"))]
  Info {
    ids: Vec<u32>,
  },
  Sync,
  Transcode,
}

impl Subcommand {
  #[throws]
  pub(crate) fn run(self) {
    use tracing_subscriber::fmt::Layer;

    let library = Library::new()?;

    let appender = tracing_appender::rolling::never(library.base_dir(), "log.txt");

    let (non_blocking, _guard) = tracing_appender::non_blocking(appender);

    let filter = EnvFilter::from_default_env()
      .add_directive("music=info".parse()?)
      .add_directive("warn".parse()?);

    let subscriber = tracing_subscriber::registry()
      .with(filter)
      .with(Layer::new())
      .with(Layer::new().with_writer(non_blocking));

    LogTracer::init()?;

    tracing::subscriber::set_global_default(subscriber)?;

    match self {
      Self::Import => Self::import(&library)?,
      Self::FixTranscodeTags => Self::fix_transcode_tags(&library)?,
      Self::Info { ids } => Self::info(&library, &ids)?,
      Self::Sync => Self::sync(&library)?,
      Self::Transcode => Self::transcode(&library)?,
    }
  }

  #[throws]
  fn info(library: &Library, ids: &[u32]) {
    for id in ids {
      let id = Id::new(*id);
      {
        let mp3 = Mp3::from_id(id);
        let path = library.mp3_path(mp3);
        if !path.exists() {
          continue;
        }

        let tag = id3::Tag::read_from_path(&path)?;

        for frame in tag.frames() {
          eprint!("{}: ", frame.name());
          use id3::Content::*;
          match frame.content() {
            Unknown(bytes) => eprintln!("Unknown: {:?}", String::from_utf8_lossy(&bytes)),
            Picture(picture) => eprintln!("Picture {}", picture.description),
            Text(text) => eprintln!("{}", text),
            ExtendedText(item) => eprintln!("{:?} {:?}", item.description, item.value),
            Comment(comment) => eprintln!("{} {}", comment.description, comment.text),
            _ => eprintln!("Unsupported content type"),
          }
        }
      }
    }
  }

  #[throws]
  fn fix_transcode_tags(library: &Library) {
    let span = span!(Level::INFO, "tag");
    let _guard = span.enter();

    for mp3 in library.mp3s()?.into_iter() {
      let path = library.mp3_path(mp3);
      let mut tag = id3::Tag::read_from_path(&path)?;

      let mut trackid = None;
      let mut albumid = None;

      for extended_text in tag.extended_texts() {
        match extended_text.description.as_str() {
          "MUSICBRAINZ_ALBUMID" => {
            albumid = Some(extended_text.value.trim_matches('\0').to_owned());
          }
          "MUSICBRAINZ_TRACKID" => {
            trackid = Some(extended_text.value.trim_matches('\0').to_owned());
          }
          _ => {}
        }
      }

      if let Some(trackid) = trackid {
        trace!("Retagging {}", mp3);
        eprint!("•");

        let albumid =
          albumid.ok_or_else(|| anyhow!("Track had track ID but not album ID: {}", mp3))?;

        let ufid = format!("http://musicbrainz.org\0{}", trackid).into();
        tag.add_frame(id3::Frame::with_content(
          "UFID",
          id3::Content::Unknown(ufid),
        ));

        tag.add_extended_text("MusicBrainz Album Id", format!("{}\0", albumid));

        tag.write_to_path(&path, id3::Version::Id3v24)?;
      } else {
        trace!("Skipping {}", mp3);
        eprint!("·");
      }
    }
    eprintln!();
  }

  #[throws]
  fn sync(library: &Library) {
    let status = Command::new("rsync")
      .args(&["-avz", "--progress", "destiny.whatbox.ca:files/"])
      .arg(library.new_dir())
      .status()
      .with_context(|| anyhow!("Failed to invoke rsync"))?;

    if !status.success() {
      bail!("rsync failed: {}", status);
    }

    let status = Command::new("open")
      .args(&["--wait-apps", "-a", "MusicBrainz Picard"])
      .arg(library.new_dir())
      .status()
      .with_context(|| anyhow!("Failed to invoke open"))?;

    if !status.success() {
      bail!("open failed: {}", status);
    }

    Self::import(library)?;

    let status = Command::new("open")
      .arg(library.new_dir())
      .status()
      .with_context(|| anyhow!("Failed to invoke open"))?;

    if !status.success() {
      bail!("open failed: {}", status);
    }
  }

  #[throws]
  fn import(library: &Library) {
    let span = span!(Level::INFO, "import");
    let _guard = span.enter();

    info!("Looking for new tracks…");

    let mut mp3s = 0;
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
        }
        _ => others += 1,
      };
    }

    info!("Found {} MP3s.", mp3s);
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

    if clusterizer.cluster_count() == 0 {
      info!("No tracks to import");
      return;
    }

    info!("Found {} clusters:", clusterizer.cluster_count());

    let mut imported = Vec::new();

    for cluster_key in clusterizer.keys() {
      info!("- {}", cluster_key);
    }

    let input: String = input().msg("Do these clusters look okay? ").get();

    if &input[..1] != "y" {
      info!("Cancelling…");
      return;
    }

    {
      let mut next_id = next_id;
      for cluster in clusterizer.clusters() {
        for import in cluster.imports() {
          let destination = import.destination(library, next_id);
          fs::rename(&import.path(), &destination)?;
          imported.push(destination);
          next_id = next_id.next();
        }
      }
    }

    info!("Opening with Music…");

    let status = Command::new("open")
      .arg("-a")
      .arg("Music")
      .args(imported)
      .status()
      .unwrap();

    if !status.success() {
      process::exit(status.code().unwrap_or(1))
    }
  }

  #[throws]
  fn transcode(library: &Library) {
    let span = span!(Level::INFO, "transcode");
    let _guard = span.enter();

    let mut flacs = BTreeSet::new();

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

      if extension.as_str() == "flac" {
        flacs.insert(path);
      }
    }

    let total = flacs.len();
    let total_width = total.to_string().len();
    info!("{} FLACs to transcode…", total);

    let tmpdir = TempDir::new().unwrap();
    let i = AtomicUsize::new(0);
    let n = AtomicUsize::new(0);

    let results = flacs
      .par_iter()
      .map(|src| {
        let src = src.to_path_buf();
        let i = i.fetch_add(1, Ordering::Relaxed);
        let tmp = tmpdir.path().join(format!("{}.mp3", i));
        let dst = src.with_extension("mp3");

        Command::new("ffmpeg")
          .arg("-i")
          .arg(&src)
          .arg("-qscale:a")
          .arg("0")
          .arg(&tmp)
          .output()
          .with_context(|| anyhow!("Failed to invoke ffmpeg"))
          .and_then(|output| {
            let n = n.fetch_add(1, Ordering::Relaxed);
            let count = format!("{:width$}/{}", n + 1, total, width = total_width);
            let src_file_name = src.file_name().unwrap_or_default().to_string_lossy();
            if output.status.success() {
              info!("{} [+] {}", count, src_file_name);
              Ok((src, tmp, dst))
            } else {
              error!("{} [x] {}", count, src_file_name);
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
      info!("Renaming `{}` to `{}`", tmp.display(), dst.display());
      fs::rename(&tmp, &dst)?;
    }
  }
}
