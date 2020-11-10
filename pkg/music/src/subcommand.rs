use crate::common::*;

#[derive(StructOpt)]
pub(crate) enum Subcommand {
  MbidToNumbered,
  Import { paths: Vec<PathBuf> },
  Transcode,
}

impl Subcommand {
  pub(crate) fn run(self) -> Result<()> {
    match self {
      Self::MbidToNumbered => Self::mbid_to_numbered(),
      Self::Import { paths } => Self::import(&paths),
      Self::Transcode => Self::transcode(),
    }
  }

  fn transcode() -> Result<()> {
    Ok(())
  }

  fn import(paths: &[PathBuf]) -> Result<()> {
    let mut imports = Vec::new();

    for path in paths {
      for result in WalkDir::new(path) {
        let entry = result?;

        if entry.file_type().is_dir() {
          continue;
        }

        let path = entry.path();

        let extension = path
          .extension()
          .unwrap_or_default()
          .to_string_lossy()
          .into_owned();

        match extension.as_str() {
          "flac" => imports.push(Import::new(path)?),
          _ => eprintln!("Skipping {}…", path.display()),
        }
      }
    }

    let mut albums = BTreeMap::new();

    for import in &imports {
      let album = albums.entry(import.album_key()).or_insert_with(Vec::new);
      album.push(import);
      album.sort_by_key(|import| import.track);
    }

    let next_id = library::next_id()?;

    let mut id = next_id;
    eprintln!("Found {} albums.", albums.len());
    for ((artist, album), imports) in &albums {
      eprintln!("{} by {}", album, artist);

      for (i, import) in imports.iter().enumerate() {
        assert_eq!(i, import.track as usize);
        eprintln!("{:02} - {}", import.track, import.title);
        assert!(!library::flac(id as u32)?.exists());
        id += 1;
      }
    }

    eprintln!("Renaming {} tracks…", albums.values().flatten().count());
    for (i, import) in albums.values().flatten().enumerate() {
      fs::rename(&import.path, &library::flac(next_id + i as u32)?)?;
    }

    Self::transcode()?;

    Ok(())
  }

  fn mbid_to_numbered() -> Result<()> {
    let home = dirs::home_dir().ok_or(Error::HomeDir)?;

    let music = home.join("Dropbox/music");
    let itunes = music.join("iTunes");

    let mut flac = BTreeSet::new();
    let mut mp3 = BTreeSet::new();
    let mut other = BTreeSet::new();
    let mut ignore = BTreeSet::new();
    let mut m4a = BTreeSet::new();

    let filter = |entry: &DirEntry| !entry.path().starts_with(&itunes);

    for result in WalkDir::new(&music).into_iter().filter_entry(filter) {
      let entry = result?;

      if entry.file_type().is_dir() {
        continue;
      }

      let path = entry.path().to_path_buf();

      if path.file_name() == Some(OsStr::new(".DS_Store")) {
        continue;
      }

      let extension = path
        .extension()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned();

      match extension.as_str() {
        "mp3" => mp3.insert(path),
        "flac" => flac.insert(path),
        "m4a" => m4a.insert(path),
        "m3u" | "cue" | "log" | "png" | "jpg" => ignore.insert(path),
        _ => other.insert(path),
      };
    }

    let mut pairs = BTreeSet::new();

    for master in &flac {
      let mut a = music.join("library");
      a.push(master.file_name().unwrap());
      a.set_extension("mp3");

      let mut b = master.to_owned();
      b.set_extension("flac.mp3");

      let a_exists = mp3.contains(&a);
      let b_exists = mp3.contains(&b);

      let transcode = match (a_exists, b_exists) {
        (true, true) => return Err(Error::DuplicateTranscodes { a, b }),
        (false, false) => continue,
        (true, false) => a,
        (false, true) => b,
      };

      pairs.insert((transcode, master.to_owned()));
    }

    for (transcode, master) in &pairs {
      assert!(flac.contains(master));
      flac.remove(master);
      assert!(mp3.contains(transcode));
      mp3.remove(transcode);
    }

    assert!(flac.is_empty());
    assert!(other.is_empty());

    eprintln!("{} pairs", pairs.len());
    eprintln!("{} mp3s", mp3.len());

    let tracks = mp3
      .into_iter()
      .map(|mp3| (mp3, None))
      .chain(pairs.into_iter().map(|(mp3, flac)| (mp3, Some(flac))))
      .map(|(mp3, flac)| Track::new(mp3, flac))
      .collect::<Result<Vec<Track>>>()?;

    {
      let mut albums = BTreeMap::new();

      for track in &tracks {
        let album = albums.entry(track.album_key()).or_insert(BTreeMap::new());

        if album.contains_key(&track.track_key()) {
          eprintln!("{}", track.mp3.display());
        }

        album.insert(track.track_key(), track);
      }

      for album in albums.values() {
        let mut last_disc = 0;
        let mut last_track_number = 0;
        let mut gap = false;

        for ((disc, track_number), track) in album {
          if *disc != last_disc {
            if *disc != last_disc + 1 {
              gap = true;
            } else {
              last_track_number = 0;
            }
          }

          if *track_number != last_track_number {
            if *track_number != last_track_number + 1 {
              gap = true;
            }
          }

          if gap {
            eprintln!(
              "{}/{} {} {}",
              disc, track_number, track.album, track.album_artist
            );
          }

          last_disc = *disc;
          last_track_number = *track_number;
          gap = false;
        }
      }
    }

    let sequence = tracks
      .iter()
      .map(|track| {
        (
          (
            track.added,
            track.album_artist.as_str(),
            track.album.as_str(),
            track.disc,
            track.track,
          ),
          track,
        )
      })
      .collect::<BTreeMap<(DateTime<Utc>, &str, &str, u32, u32), &Track>>();

    let new = home.join("Dropbox/new");

    for (i, track) in sequence.values().enumerate() {
      let stem = format!("{:06}", i);

      {
        let mut dst = new.clone();
        dst.push("mp3");
        dst.push(&stem);
        dst.set_extension("mp3");
        fs::rename(&track.mp3, &dst)?;
      }

      if let Some(flac) = &track.flac {
        let mut dst = new.clone();
        dst.push("flac");
        dst.push(&stem);
        dst.set_extension("flac");
        fs::rename(&flac, &dst)?;
      }
    }

    Ok(())
  }
}
