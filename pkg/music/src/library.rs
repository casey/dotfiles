use crate::common::*;

fn id_string(id: u32) -> String {
  format!("{:06}", id)
}

fn flac_filename(id: u32) -> String {
  let mut s = id_string(id);
  s.push_str(".flac");
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

pub(crate) fn next_id() -> Result<u32> {
  let mp3s = mp3s()?;

  Ok(mp3s.len() as u32)
}

pub(crate) fn mp3s() -> Result<Vec<PathBuf>> {
  let dir = mp3_dir()?;

  let mut mp3s = Vec::new();

  let mut id = 0;
  for result in WalkDir::new(dir).sort_by(|a, b| a.path().cmp(b.path())) {
    let entry = result?;

    if entry.file_type().is_dir() {
      continue;
    }

    let path = entry.path().to_path_buf();

    let file_name = path.file_name().unwrap();

    if file_name == OsStr::new(".DS_Store") {
      continue;
    }

    assert_eq!(file_name, OsStr::new(&format!("{:06}.mp3", id)));
    mp3s.push(path);
    id += 1;
  }

  Ok(mp3s)
}
