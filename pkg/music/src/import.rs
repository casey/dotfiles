use crate::common::*;

#[derive(Debug)]
pub(crate) struct Import {
  pub(crate) path:         PathBuf,
  pub(crate) track:        u32,
  pub(crate) album_artist: String,
  pub(crate) album:        String,
  pub(crate) title:        String,
}

impl Import {
  pub(crate) fn new(path: &Path) -> Result<Self> {
    assert_eq!(path.extension(), Some(OsStr::new("flac")));

    let tag = metaflac::Tag::read_from_path(path).context(error::Flac { path })?;

    let comment = tag
      .vorbis_comments()
      .ok_or_else(|| Error::VorbisCommentMissing {
        path: path.to_owned(),
      })?;

    let album_artist = comment.album_artist().unwrap()[0].clone();
    let album = comment.album().unwrap()[0].clone();
    let track = comment.track().unwrap() - 1;
    let title = comment.title().unwrap()[0].clone();

    Ok(Import {
      path: path.to_owned(),
      album_artist,
      title,
      album,
      track,
    })
  }

  pub(crate) fn album_key(&self) -> (&str, &str) {
    (&self.album_artist, &self.album)
  }
}
