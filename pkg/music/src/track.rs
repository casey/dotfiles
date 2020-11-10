use crate::common::*;

pub(crate) struct Track {
  pub(crate) source:       Source,
  pub(crate) added:        DateTime<Utc>,
  pub(crate) disc:         u32,
  pub(crate) track:        u32,
  pub(crate) album:        String,
  pub(crate) album_artist: String,
  pub(crate) year:         i32,
  pub(crate) id:           u32,
}

impl Track {
  fn parse_id(path: &Path) -> Result<u32> {
    let file_stem = path.file_stem().ok_or_else(|| Error::TrackName {
      path: path.to_owned(),
    })?;

    file_stem
      .to_string_lossy()
      .parse::<u32>()
      .map_err(|_| Error::TrackName {
        path: path.to_owned(),
      })
  }

  fn new(source: Source) -> Result<Track> {
    let added: DateTime<Utc>;
    let disc;
    let track;
    let album;
    let year;
    let album_artist;
    let id;
    match &source {
      Source::Flac(flac) => {
        id = Self::parse_id(flac)?;
        todo!();
      },
      Source::Mp3(mp3) | Source::Both { mp3, .. } => {
        id = Self::parse_id(mp3)?;

        let tag = Tag::read_from_path(&mp3).context(error::Id3 { path: &mp3 })?;

        let comments = tag
          .comments()
          .filter(|comment| comment.description.is_empty())
          .map(|comment| comment.text.clone())
          .collect::<Vec<String>>();

        if comments.len() != 1 {
          return Err(Error::Comments {
            path: mp3.to_owned(),
            comments,
          });
        }

        let timestamp =
          comments[0]
            .trim_matches('\u{0}')
            .parse::<u64>()
            .context(error::Timestamp {
              path: &mp3,
              text: &comments[0],
            })?;

        let naive = NaiveDateTime::from_timestamp(timestamp as i64, 0);

        added = DateTime::from_utc(naive, Utc);

        disc = {
          let number = tag.disc();

          match number {
            Some(0) | None =>
              return Err(Error::DiscNumber {
                path: mp3.to_owned(),
                number,
              }),
            Some(n) => n - 1,
          }
        };

        track = {
          let number = tag.track();

          match number {
            Some(0) | None =>
              return Err(Error::TrackNumber {
                path: mp3.to_owned(),
                number,
              }),
            Some(n) => n - 1,
          }
        };

        album = tag
          .album()
          .ok_or_else(|| Error::TrackAlbum {
            path: mp3.to_owned(),
          })?
          .to_owned();

        if album.is_empty() {
          return Err(Error::TrackAlbum {
            path: mp3.to_owned(),
          });
        }

        album_artist = tag
          .album_artist()
          .ok_or_else(|| Error::TrackAlbumArtist {
            path: mp3.to_owned(),
          })?
          .to_owned();

        if album_artist.is_empty() {
          return Err(Error::TrackAlbumArtist {
            path: mp3.to_owned(),
          });
        }

        year = tag
          .date_recorded()
          .map(|timestamp| timestamp.year)
          .or_else(|| tag.year())
          .ok_or_else(|| Error::TrackYear { path: mp3.clone() })?;
      },
    }

    Ok(Track {
      id,
      source,
      added,
      disc,
      track,
      album,
      year,
      album_artist,
    })
  }

  pub(crate) fn album_key(&self) -> (i32, &str, &str) {
    (self.year, &self.album, &self.album_artist)
  }

  pub(crate) fn track_key(&self) -> (u32, u32) {
    (self.disc, self.track)
  }

  pub(crate) fn mp3(&self) -> Option<&Path> {
    match &self.source {
      Source::Mp3(mp3) | Source::Both { mp3, .. } => Some(mp3),
      Source::Flac(_) => None,
    }
  }

  pub(crate) fn flac(&self) -> Option<&Path> {
    match &self.source {
      Source::Flac(flac) | Source::Both { flac, .. } => Some(flac),
      Source::Mp3(_) => None,
    }
  }
}

impl TryFrom<Source> for Track {
  type Error = Error;

  fn try_from(source: Source) -> Result<Self, Self::Error> {
    Track::new(source)
  }
}
