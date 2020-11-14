use crate::common::*;

#[derive(Debug)]
pub(crate) struct Import {
  album:        String,
  album_artist: String,
  format:       Format,
  path:         PathBuf,
  title:        String,
  track_number: u32,
  disc_number:  u32,
}

impl Import {
  pub(crate) fn album_artist(&self) -> &str {
    &self.album_artist
  }

  pub(crate) fn album(&self) -> &str {
    &self.album
  }

  pub(crate) fn track_number(&self) -> u32 {
    self.track_number
  }

  pub(crate) fn disc_number(&self) -> u32 {
    self.disc_number
  }

  pub(crate) fn path(&self) -> &Path {
    &self.path
  }

  pub(crate) fn format(&self) -> Format {
    self.format
  }

  pub(crate) fn destination(&self, library: &Library, id: Id) -> PathBuf {
    match self.format {
      Format::Mp3 => library.mp3_path(Mp3::from_id(id)),
      Format::Flac => library.flac_path(Flac::from_id(id)),
    }
  }
}

impl TryFrom<&Path> for Import {
  type Error = Error;

  #[throws]
  fn try_from(path: &Path) -> Self {
    let format = path.try_into()?;
    let path = path.to_owned();

    let album;
    let album_artist;
    let disc_number;
    let title;
    let track_number;
    match format {
      Format::Mp3 => {
        let tag = id3::Tag::read_from_path(&path)?;

        let missing = || anyhow!("FLAC file missing metadata: `{}`", path.display());

        title = tag.title().ok_or_else(missing)?.to_owned();
        disc_number = tag.disc().ok_or_else(missing)? - 1;
        track_number = tag.track().ok_or_else(missing)? - 1;
        album = tag.album().ok_or_else(missing)?.to_owned();
        album_artist = tag.album_artist().ok_or_else(missing)?.to_owned();
      },
      Format::Flac => {
        let tag = metaflac::Tag::read_from_path(&path)?;
        let comment = tag
          .vorbis_comments()
          .ok_or_else(|| anyhow!("FLAC file missing Vorbis comments: `{}`", path.display()))?;

        let missing = || anyhow!("FLAC file missing metadata: `{}`", path.display());

        album = comment.album().ok_or_else(missing)?[0].clone();
        album_artist = comment.album_artist().ok_or_else(missing)?[0].clone();
        disc_number = comment.get("DISCNUMBER").ok_or_else(missing)?[0]
          .parse::<u32>()
          .with_context(|| anyhow!("Failed to parse FLAC disc number: {}", path.display()))?
          - 1;
        title = comment.title().ok_or_else(missing)?[0].clone();
        track_number = comment.track().ok_or_else(missing)? - 1;
      },
    }

    Self {
      path,
      format,
      album,
      album_artist,
      title,
      track_number,
      disc_number,
    }
  }
}
