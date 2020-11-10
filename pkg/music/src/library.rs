use crate::common::*;

fn id_string(id: u32) -> String {
  format!("{:06}", id)
}

fn flac_filename(id: u32) -> String {
  let mut s = id_string(id);
  s.push_str(".flac");
  s
}

fn mp3_filename(id: u32) -> String {
  let mut s = id_string(id);
  s.push_str(".mp3");
  s
}

pub(crate) fn base() -> Result<PathBuf> {
  Ok(
    dirs::home_dir()
      .ok_or(Error::HomeDir)?
      .join("Dropbox/music"),
  )
}

pub(crate) fn flac_dir() -> Result<PathBuf> {
  Ok(base()?.join("flac"))
}

pub(crate) fn mp3_dir() -> Result<PathBuf> {
  Ok(base()?.join("mp3"))
}

pub(crate) fn flac(id: u32) -> Result<PathBuf> {
  Ok(flac_dir()?.join(flac_filename(id)))
}

pub(crate) fn mp3(id: u32) -> Result<PathBuf> {
  Ok(flac_dir()?.join(mp3_filename(id)))
}

pub(crate) fn next_id() -> Result<u32> {
  let mp3s = mp3s()?;

  Ok(mp3s.len() as u32)
}

pub(crate) fn mp3s() -> Result<Vec<PathBuf>> {
  let dir = mp3_dir()?;

  let mut mp3s = Vec::new();

  for result in WalkDir::new(dir).sort_by(|a, b| a.path().cmp(b.path())) {
    let entry = result?;

    if entry.file_type().is_dir() {
      continue;
    }

    let path = entry.path().to_path_buf();

    let file_name = path.file_name().unwrap().to_string_lossy().into_owned();

    if file_name == ".DS_Store" {
      continue;
    }

    assert_eq!(path.extension().unwrap().to_string_lossy(), "mp3");

    mp3s.push(path);
  }

  Ok(mp3s)
}

pub(crate) fn flacs() -> Result<Vec<PathBuf>> {
  let dir = flac_dir()?;

  let mut flacs = Vec::new();

  for result in WalkDir::new(dir).sort_by(|a, b| a.path().cmp(b.path())) {
    let entry = result?;

    if entry.file_type().is_dir() {
      continue;
    }

    let path = entry.path().to_path_buf();

    let file_name = path.file_name().unwrap().to_string_lossy().into_owned();

    if file_name == ".DS_Store" {
      continue;
    }

    let (id, extension) = file_name.split_at(6);

    assert_eq!(extension, ".flac");

    id.parse::<u64>().unwrap();

    flacs.push(path);
  }

  Ok(flacs)
}

pub(crate) fn sources() -> Result<Vec<Source>> {
  let mut mp3s = self::mp3s()?.into_iter().collect::<BTreeSet<PathBuf>>();

  let mut sources = Vec::new();

  for flac in self::flacs()? {
    let file_name = flac.file_name().unwrap();
    let mut mp3 = mp3_dir()?.join(file_name);
    mp3.set_extension("mp3");

    if let Some(mp3) = mp3s.take(&mp3) {
      sources.push(Source::both(flac, mp3))
    } else {
      sources.push(Source::flac(flac));
    }
  }

  for mp3 in mp3s {
    sources.push(Source::mp3(mp3));
  }

  Ok(sources)
}

pub(crate) fn tracks() -> Result<Vec<Track>> {
  sources()?.into_iter().map(TryInto::try_into).collect()
}
